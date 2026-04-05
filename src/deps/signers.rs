use std::sync::{Arc, OnceLock};

use libsigners::{HS256Signer, RS256Validator, Sign, Validate};

use crate::di::{AsyncFromEnv, FromEnv, ProviderRegistry};

static VALIDATOR_REGISTRY: OnceLock<ProviderRegistry<dyn Validate>> = OnceLock::new();

pub fn validator_registry() -> &'static ProviderRegistry<dyn Validate> {
    VALIDATOR_REGISTRY.get_or_init(|| {
        let reg = ProviderRegistry::<dyn Validate>::new("VALIDATOR_TYPE");
        reg.register("jwt", |ctx| {
            Ok(Arc::new(HS256Signer::new(ctx.get("aud")?.to_owned())))
        })
        .unwrap();
        reg.register("basic", |ctx| {
            Ok(Arc::new(RS256Validator::new(
                ctx.get("public_key")?.to_string(),
                ctx.get("aud")?.to_string(),
            )))
        })
        .unwrap();
        reg.set_default("jwt").unwrap();
        reg
    })
}

impl AsyncFromEnv for Arc<dyn Validate> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, crate::di::EnvError> {
        validator_registry().resolve(ctx)
    }
}
