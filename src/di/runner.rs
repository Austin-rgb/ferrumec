use crate::di::async_from_env::AsyncFromEnvTuple;

use super::{context::EnvContext, error::EnvError, handler::Handler, tuple::FromEnvTuple};

pub fn run<F, Args>(f: F) -> Result<F::Output, EnvError>
where
    F: Handler<Args>,
    Args: FromEnvTuple,
{
    let ctx = EnvContext::default();
    let args = Args::from_env_tuple(&ctx)?;
    Ok(f.call(args))
}

pub async fn run_async<F, Args>(f: F) -> Result<F::Output, EnvError>
where
    F: Handler<Args>,
    Args: AsyncFromEnvTuple,
{
    let ctx = EnvContext::default();
    let args = Args::from_env_tuple(&ctx).await?;
    Ok(f.call(args))
}

pub fn run_with_ctx<F, Args>(f: F, ctx: &EnvContext) -> Result<F::Output, EnvError>
where
    F: Handler<Args>,
    Args: FromEnvTuple,
{
    let args = Args::from_env_tuple(ctx)?;
    Ok(f.call(args))
}
