use anyhow::Result;

use super::Claims;

pub trait Sign: Send + Sync + 'static {
    fn sign(&self, claims: &Claims) -> Result<String>;
}

pub trait Validate: Send + Sync + 'static {
    fn validate(&self, token: &str) -> Result<Claims>;
}
