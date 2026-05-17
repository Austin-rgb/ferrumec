use super::simple::ExtractFrom;
use super::{context::EnvContext, error::EnvError, handler::Handler};
use crate::di::simple::AsyncFrom;

pub fn run<F, Args>(f: F) -> Result<F::Output, EnvError>
where
    F: Handler<Args>,
    Args: ExtractFrom<EnvContext, EnvError>,
{
    let ctx = EnvContext::default();
    Ok(run_with_ctx(f, ctx)?)
}

pub async fn run_async<F, Args>(f: F) -> Result<F::Output, EnvError>
where
    F: Handler<Args>,
    Args: AsyncFrom<EnvContext, EnvError>,
{
    let ctx = EnvContext::default();
    Ok(async_run_with_ctx(f, ctx).await?)
}

pub fn run_with_ctx<F, Args, T, E>(f: F, ctx: T) -> Result<F::Output, E>
where
    F: Handler<Args>,
    Args: ExtractFrom<T, E>,
{
    let args = Args::extract_from(&ctx)?;
    Ok(f.call(args))
}

pub async fn async_run_with_ctx<F, Args, T, E>(f: F, ctx: T) -> Result<F::Output, E>
where
    F: Handler<Args>,
    Args: AsyncFrom<T, E>,
{
    let args = Args::async_from(&ctx).await?;
    Ok(f.call(args))
}
