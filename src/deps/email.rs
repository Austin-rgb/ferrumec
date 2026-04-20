use crate::email::{Sender, Resend};

use crate::di::{AsyncFromEnv, EnvError};

impl AsyncFromEnv for Arc<dyn Sender> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, crate::di::>
        let signer_type = ctx.get("emailer.type")?;
        match signer_type {
            "resend" => {
                Ok(Arc::new(Resend(ctx.get("emailer.api")?.to_owned()))>
            }
            "brevo" => Ok(Arc::new(Brevo(
                ctx.get("emailer.api")?.to_string(),
            )) as Arc<dyn Sign>),
            _ => Err(EnvError::new(format!(
                "Unsupported sign.type value: {signer_type}"
            ))),
        }
    }
}

