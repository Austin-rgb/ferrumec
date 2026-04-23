use std::sync::Arc;

use libsigners::{HS256Signer, RS256Validator,  RS256Signer};
pub use libsigners::{Validate,Sign};
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

impl AsyncFromEnv for Arc<dyn Sign> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, crate::di::EnvError> {
        let signer_type = ctx.get("sign.type")?;
        match signer_type {
            "hs256" => {
                Ok(Arc::new(HS256Signer::new(ctx.get("sign.aud")?.to_owned())) as Arc<dyn Sign>)
            }
            "rs256" => Ok(Arc::new(RS256Signer::new(
                ctx.get("sign.private_key")?.to_string(),
                ctx.get("sign.aud")?.to_string(),
            )) as Arc<dyn Sign>),
            _ => Err(EnvError::new(format!(
                "Unsupported sign.type value: {signer_type}"
            ))),
        }
    }
}

