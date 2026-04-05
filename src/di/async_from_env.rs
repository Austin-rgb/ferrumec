use super::{context::EnvContext, error::EnvError, from_env::AsyncFromEnv};

pub trait AsyncFromEnvTuple: Sized {
    async fn from_env_tuple(ctx: &EnvContext) -> Result<Self, EnvError>;
}

impl AsyncFromEnvTuple for () {
    async fn from_env_tuple(_: &EnvContext) -> Result<Self, EnvError> {
        Ok(())
    }
}

impl<A: AsyncFromEnv> AsyncFromEnvTuple for (A,) {
    async fn from_env_tuple(ctx: &EnvContext) -> Result<Self, EnvError> {
        Ok((A::from_env(ctx).await?,))
    }
}

impl<A: AsyncFromEnv, B: AsyncFromEnv> AsyncFromEnvTuple for (A, B) {
    async fn from_env_tuple(ctx: &EnvContext) -> Result<Self, EnvError> {
        Ok((A::from_env(ctx).await?, B::from_env(ctx).await?))
    }
}

impl<A: AsyncFromEnv, B: AsyncFromEnv, C: AsyncFromEnv> AsyncFromEnvTuple for (A, B, C) {
    async fn from_env_tuple(ctx: &EnvContext) -> Result<Self, EnvError> {
        Ok((
            A::from_env(ctx).await?,
            B::from_env(ctx).await?,
            C::from_env(ctx).await?,
        ))
    }
}
