use super::error::EnvError;
use std::collections::HashMap;
use std::env;

pub struct EnvContext {
    vars: HashMap<String, String>,
}

impl EnvContext {
    pub fn from_system() -> Self {
        Self {
            vars: env::vars().collect(),
        }
    }

    pub fn with_vars(vars: HashMap<String, String>) -> Self {
        Self { vars }
    }

    pub fn get(&self, key: &str) -> Result<&str, EnvError> {
        self.vars
            .get(key)
            .map(String::as_str)
            .ok_or_else(|| EnvError::new(format!("Missing env var: {key}")))
    }

    pub fn get_optional(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(String::as_str)
    }
}

impl Default for EnvContext {
    fn default() -> Self {
        Self::from_system()
    }
}
