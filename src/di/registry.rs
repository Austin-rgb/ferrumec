use super::{context::EnvContext, error::EnvError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type Factory<T> = Arc<dyn Fn(&EnvContext) -> Result<Arc<T>, EnvError> + Send + Sync>;

struct Inner<T: ?Sized> {
    factories: HashMap<String, Factory<T>>,
    env_key: String,
    default_key: Option<String>,
    frozen: bool,
}

pub struct ProviderRegistry<T: ?Sized> {
    inner: RwLock<Inner<T>>,
}

impl<T: ?Sized + 'static> ProviderRegistry<T> {
    pub fn new(env_key: impl Into<String>) -> Self {
        Self {
            inner: RwLock::new(Inner {
                factories: HashMap::new(),
                env_key: env_key.into(),
                default_key: None,
                frozen: false,
            }),
        }
    }

    fn write_guard(&self) -> Result<std::sync::RwLockWriteGuard<'_, Inner<T>>, EnvError> {
        let g = self
            .inner
            .write()
            .map_err(|e| EnvError::with_source("Lock poisoned", e.to_string()))?;
        if g.frozen {
            return Err(EnvError::new(
                "Registry is frozen; no further mutations allowed",
            ));
        }
        Ok(g)
    }

    fn read_guard(&self) -> Result<std::sync::RwLockReadGuard<'_, Inner<T>>, EnvError> {
        self.inner
            .read()
            .map_err(|e| EnvError::with_source("Lock poisoned", e.to_string()))
    }

    pub fn register<F>(&self, key: impl Into<String>, factory: F) -> Result<(), EnvError>
    where
        F: Fn(&EnvContext) -> Result<Arc<T>, EnvError> + Send + Sync + 'static,
    {
        self.write_guard()?
            .factories
            .insert(key.into(), Arc::new(factory));
        Ok(())
    }

    pub fn set_default(&self, key: impl Into<String>) -> Result<(), EnvError> {
        self.write_guard()?.default_key = Some(key.into());
        Ok(())
    }

    pub fn remove_provider(&self, key: &str) -> Result<(), EnvError> {
        self.write_guard()?.factories.remove(key);
        Ok(())
    }

    pub fn clear(&self) -> Result<(), EnvError> {
        self.write_guard()?.factories.clear();
        Ok(())
    }

    pub fn freeze(&self) -> Result<(), EnvError> {
        self.inner
            .write()
            .map_err(|e| EnvError::with_source("Lock poisoned", e.to_string()))?
            .frozen = true;
        Ok(())
    }

    pub fn has_provider(&self, key: &str) -> Result<bool, EnvError> {
        Ok(self.read_guard()?.factories.contains_key(key))
    }

    pub fn available_providers(&self) -> Result<Vec<String>, EnvError> {
        let mut keys: Vec<String> = self.read_guard()?.factories.keys().cloned().collect();
        keys.sort();
        Ok(keys)
    }

    pub fn resolve(&self, ctx: &EnvContext) -> Result<Arc<T>, EnvError> {
        let inner = self.read_guard()?;

        let key = ctx
            .get_optional(&inner.env_key)
            .map(String::from)
            .or_else(|| inner.default_key.clone())
            .ok_or_else(|| {
                EnvError::new(format!(
                    "No provider key: set '{}' or register a default",
                    inner.env_key
                ))
            })?;

        let factory = inner.factories.get(&key).ok_or_else(|| {
            let available: Vec<_> = inner.factories.keys().collect();
            EnvError::new(format!(
                "Unknown provider '{key}'. Available: {available:?}"
            ))
        })?;

        let factory = Arc::clone(factory);
        drop(inner);
        factory(ctx)
    }
}
