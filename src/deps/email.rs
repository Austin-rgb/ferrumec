use crate::di::{AsyncFromEnv, EnvError};
use crate::email::{Brevo, Resend, Sender};
use std::sync::Arc;

impl AsyncFromEnv for Arc<dyn Sender> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, EnvError> {
        let signer_type = ctx.get("emailer.type")?;
        match signer_type {
            "resend" => Ok(Arc::new(Resend(ctx.get("emailer.api")?.to_owned()))),
            "brevo" => Ok(Arc::new(Brevo(ctx.get("emailer.api")?.to_string()))),
            _ => Err(EnvError::new(format!(
                "Unsupported sign.type value: {signer_type}"
            ))),
        }
    }
}
