//! # LayeredCache
//!
//! A cache with a fast **Moka** local tier and an optional **Redis** remote
//! tier. Reads check local first; on a miss the remote tier (if configured)
//! is consulted and the result is back-filled into the local cache.
//!
//! ## Local-only (no Redis)
//!
//! ```rust,no_run
//! use layered_cache::{LayeredCache, LayeredCacheConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let cache = LayeredCache::new(
//!         LayeredCacheConfig::default()
//!             .local_capacity(10_000)
//!             .local_ttl(Duration::from_secs(60)),
//!     ).await?;
//!
//!     cache.set("hello", "world").await?;
//!     let val: Option<String> = cache.get("hello").await?;
//!     cache.delete("hello").await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## With Redis
//!
//! ```rust,no_run
//! use layered_cache::{LayeredCache, LayeredCacheConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let cache = LayeredCache::new(
//!         LayeredCacheConfig::default()
//!             .local_ttl(Duration::from_secs(30))
//!             .redis("redis://127.0.0.1/")
//!             .redis_ttl(Duration::from_secs(300)),
//!     ).await?;
//!
//!     cache.set("hello", "world").await?;
//!     let val: Option<String> = cache.get("hello").await?;
//!
//!     Ok(())
//! }
//! ```

use anyhow::Result;
use moka::future::Cache as MokaCache;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

// ── Config ───────────────────────────────────────────────────────────────────

/// Builder-style configuration for [`LayeredCache`].
///
/// Redis is **opt-in**: call `.redis(url)` to enable the remote tier.
#[derive(Clone, Debug, Default)]
pub struct LayeredCacheConfig {
    /// Optional Redis connection URL, e.g. `"redis://127.0.0.1/"`.
    redis_url: Option<String>,
    /// Maximum number of entries kept in the local (Moka) cache.
    local_capacity: Option<u64>,
    /// How long a local entry lives before being evicted.
    local_ttl: Option<Duration>,
    /// TTL applied to Redis keys via `SETEX`. `None` means no expiry.
    redis_ttl: Option<Duration>,
}

impl LayeredCacheConfig {
    /// Enable the Redis remote tier.
    pub fn redis(mut self, url: impl Into<String>) -> Self {
        self.redis_url = Some(url.into());
        self
    }

    /// Maximum number of entries in the local cache (default: 10 000).
    pub fn local_capacity(mut self, cap: u64) -> Self {
        self.local_capacity = Some(cap);
        self
    }

    /// Per-entry TTL for the local (Moka) cache.
    pub fn local_ttl(mut self, ttl: Duration) -> Self {
        self.local_ttl = Some(ttl);
        self
    }

    /// Disable per-entry TTL in the local cache.
    pub fn no_local_ttl(mut self) -> Self {
        self.local_ttl = None;
        self
    }

    /// TTL applied to keys written to Redis.
    pub fn redis_ttl(mut self, ttl: Duration) -> Self {
        self.redis_ttl = Some(ttl);
        self
    }

    /// Store keys in Redis without an expiry.
    pub fn no_redis_ttl(mut self) -> Self {
        self.redis_ttl = None;
        self
    }
}

// ── Cache ────────────────────────────────────────────────────────────────────

/// A cache with a local Moka tier and an optional Redis remote tier.
///
/// Values must implement [`Serialize`] + [`DeserializeOwned`].
/// They are stored as JSON strings so no type parameter is needed on the
/// struct itself.
#[derive(Clone)]
pub struct LayeredCache {
    local: MokaCache<String, String>,
    redis: Option<MultiplexedConnection>,
    redis_ttl: Option<Duration>,
}

impl LayeredCache {
    /// Build the cache, connecting to Redis only if a URL was provided.
    pub async fn new(config: LayeredCacheConfig) -> Result<Self> {
        // ── local tier ───────────────────────────────────────────────────────
        let capacity = config.local_capacity.unwrap_or(10_000);
        let mut builder = MokaCache::builder().max_capacity(capacity);
        if let Some(ttl) = config.local_ttl {
            builder = builder.time_to_live(ttl);
        }
        let local = builder.build();

        // ── remote tier (optional) ───────────────────────────────────────────
        let redis = match config.redis_url {
            Some(ref url) => {
                let client = Client::open(url.as_str())?;
                Some(client.get_multiplexed_async_connection().await?)
            }
            None => None,
        };

        Ok(Self {
            local,
            redis,
            redis_ttl: config.redis_ttl,
        })
    }

    /// Returns `true` when a Redis connection is active.
    pub fn has_redis(&self) -> bool {
        self.redis.is_some()
    }

    // ── Read ─────────────────────────────────────────────────────────────────

    /// Retrieve a value, checking the local cache first.
    ///
    /// On a local miss the remote tier is consulted (if configured) and the
    /// result is back-filled into the local cache before being returned.
    pub async fn get<V>(&self, key: &str) -> Result<Option<V>>
    where
        V: DeserializeOwned + Serialize,
    {
        // 1. Local hit — cheapest path, no I/O.
        if let Some(raw) = self.local.get(key).await {
            return Ok(Some(serde_json::from_str(&raw)?));
        }

        // 2. Remote fallback (only when Redis is configured).
        if let Some(ref conn) = self.redis {
            let mut conn = conn.clone();
            let raw: Option<String> = conn.get(key).await?;

            if let Some(ref raw) = raw {
                // Back-fill local so the next access is free.
                self.local.insert(key.to_owned(), raw.clone()).await;
                return Ok(Some(serde_json::from_str(raw)?));
            }
        }

        Ok(None)
    }

    // ── Write ─────────────────────────────────────────────────────────────────

    /// Insert a value into the local cache and, if configured, Redis.
    pub async fn set<V>(&self, key: &str, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        let raw = serde_json::to_string(value)?;

        self.local.insert(key.to_owned(), raw.clone()).await;

        if let Some(ref conn) = self.redis {
            let mut conn = conn.clone();
            match self.redis_ttl {
                Some(ttl) => conn.set_ex(key, &raw, ttl.as_secs()).await?,
                None => conn.set(key, &raw).await?,
            }
        }

        Ok(())
    }

    // ── Delete ────────────────────────────────────────────────────────────────

    /// Remove a key from all active tiers.
    pub async fn delete(&self, key: &str) -> Result<()> {
        self.local.invalidate(key).await;

        if let Some(ref conn) = self.redis {
            let mut conn = conn.clone();
            conn.del::<_, ()>(key).await?;
        }

        Ok(())
    }

    // ── Existence ─────────────────────────────────────────────────────────────

    /// Returns `true` if the key exists in any active tier.
    pub async fn exists(&self, key: &str) -> Result<bool> {
        if self.local.contains_key(key) {
            return Ok(true);
        }

        if let Some(ref conn) = self.redis {
            let mut conn = conn.clone();
            let n: u64 = conn.exists(key).await?;
            return Ok(n > 0);
        }

        Ok(false)
    }

    // ── TTL management ────────────────────────────────────────────────────────

    /// Refresh the Redis TTL of a key without altering its value.
    ///
    /// Returns `Ok(false)` when Redis is not configured or the key is absent.
    pub async fn refresh_ttl(&self, key: &str, ttl: Duration) -> Result<bool> {
        if let Some(ref conn) = self.redis {
            let mut conn = conn.clone();
            let ok: bool = conn.expire(key, ttl.as_secs() as i64).await?;
            return Ok(ok);
        }
        Ok(false)
    }

    // ── Bulk ──────────────────────────────────────────────────────────────────

    /// Flush every entry from the local cache.
    ///
    /// Redis is left untouched — useful after a deploy to force a full reload
    /// from the remote tier.
    pub fn invalidate_local_all(&self) {
        self.local.invalidate_all();
    }

    /// Number of entries currently held in the local cache.
    pub fn local_entry_count(&self) -> u64 {
        self.local.entry_count()
    }
}

