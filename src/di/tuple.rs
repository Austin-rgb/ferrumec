use super::{context::EnvContext, error::EnvError, from_env::FromEnv};

pub trait FromEnvTuple: Sized {
    fn from_env_tuple(ctx: &EnvContext) -> Result<Self, EnvError>;
}

impl FromEnvTuple for () {
    fn from_env_tuple(_: &EnvContext) -> Result<Self, EnvError> {
        Ok(())
    }
}

impl<A: FromEnv> FromEnvTuple for (A,) {
    fn from_env_tuple(ctx: &EnvContext) -> Result<Self, EnvError> {
        Ok((A::from_env(ctx)?,))
    }
}

impl<A: FromEnv, B: FromEnv> FromEnvTuple for (A, B) {
    fn from_env_tuple(ctx: &EnvContext) -> Result<Self, EnvError> {
        Ok((A::from_env(ctx)?, B::from_env(ctx)?))
    }
}

impl<A: FromEnv, B: FromEnv, C: FromEnv> FromEnvTuple for (A, B, C) {
    fn from_env_tuple(ctx: &EnvContext) -> Result<Self, EnvError> {
        Ok((A::from_env(ctx)?, B::from_env(ctx)?, C::from_env(ctx)?))
    }
}
