use std::sync::Arc;

use libsigners::{HS256Signer, RS256Signer, Sign};

use crate::di::AsyncFromEnv;

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
            _ => unreachable!("unknown signer type: {}", signer_type),
        }
    }
}
