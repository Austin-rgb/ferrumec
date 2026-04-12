use std::sync::Arc;

use crate::crypto::{HS256Signer, RS256Validator, Validate};
use crate::di::{AsyncFromEnv, EnvError};

impl AsyncFromEnv for Arc<dyn Validate> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, crate::di::EnvError> {
        let signer_type = ctx.get("validate.type")?;
        match signer_type {
            "hs256" => Ok(
                Arc::new(HS256Signer::new(ctx.get("validate.aud")?.to_owned()))
                    as Arc<dyn Validate>,
            ),
            "rs256" => Ok(Arc::new(RS256Validator::new(
                ctx.get("validate.public_key")?.to_string(),
                ctx.get("validate.aud")?.to_string(),
            )) as Arc<dyn Validate>),
            _ => Err(EnvError::new(format!(
                "Unsupported validate.type value: {signer_type}"
            ))),
        }
    }
}
