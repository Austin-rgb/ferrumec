#[derive(Debug)]
pub struct EnvError {
    pub context: String,
    pub source: Option<String>,
}

impl EnvError {
    pub fn new(c: impl Into<String>) -> Self {
        Self {
            context: c.into(),
            source: None,
        }
    }

    pub fn with_source(c: impl Into<String>, s: impl Into<String>) -> Self {
        Self {
            context: c.into(),
            source: Some(s.into()),
        }
    }
}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            Some(src) => write!(f, "{}: {}", self.context, src),
            None => write!(f, "{}", self.context),
        }
    }
}

impl std::error::Error for EnvError {}
