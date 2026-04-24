use crate::di::{AsyncFromEnv, EnvError};
pub use emailgrid::{Brevo, Resend, Sender, EmailingContext, EmailAddress};
use std::sync::Arc;
use sqlx::{Pool, Sqlite};
impl AsyncFromEnv for Arc<dyn Sender + Send + Sync> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, EnvError> {
        let sender_type = ctx.get("emailer.type")?;

        match sender_type {
            "resend" => Ok(Arc::new(Resend(ctx.get("emailer.api")?.to_string())) as Arc<dyn Sender + Send + Sync>),
            "brevo" => Ok(Arc::new(Brevo(ctx.get("emailer.api")?.to_string())) as Arc<dyn Sender + Send + Sync>),
            _ => Err(EnvError::new(format!(
                "Unsupported emailer.type value: {sender_type}"
            ))),
        }
    }
}

impl AsyncFromEnv for EmailingContext {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, EnvError> {
        let slf = EmailingContext::new(
            Arc::<dyn Sender + Send + Sync>::from_env(ctx).await?,
            Pool::<Sqlite>::from_env(ctx).await?,
            EmailAddress::from_env(ctx).await?,
        )?;
Ok(slf)
    }
}

impl AsyncFromEnv for EmailAddress {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, EnvError> {
        let email = ctx.get("email.address")?.to_string();
        let name = ctx.get("email.name")?.to_string();

        Ok(Self { email, name })
    }
}

impl From<anyhow::Error> for EnvError{
fn from(err:anyhow::Error)->Self{
let err = format!("error: {}", err);
EnvError::new(err)
}
}
