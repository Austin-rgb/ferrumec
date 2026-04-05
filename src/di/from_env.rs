use super::{context::EnvContext, error::EnvError};

pub trait FromEnv: Sized + Send + Sync + 'static {
    fn from_env(ctx: &EnvContext) -> Result<Self, EnvError>;
}

pub trait AsyncFromEnv: Sized + Send + Sync + 'static {
    async fn from_env(ctx: &EnvContext) -> Result<Self, EnvError>;
}
