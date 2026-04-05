use std::sync::{Arc, OnceLock};

use libsigners::{HS256Signer, RS256Signer, Sign};

use crate::di::{AsyncFromEnv, ProviderRegistry};

static VALIDATOR_REGISTRY: OnceLock<ProviderRegistry<dyn Sign>> = OnceLock::new();

pub fn validator_registry() -> &'static ProviderRegistry<dyn Sign> {
    VALIDATOR_REGISTRY.get_or_init(|| {
        let reg = ProviderRegistry::<dyn Sign>::new("VALIDATOR_TYPE");
        reg.register("jwt", |ctx| {
            Ok(Arc::new(HS256Signer::new(ctx.get("aud")?.to_owned())))
        })
        .unwrap();
        reg.register("basic", |ctx| {
            Ok(Arc::new(RS256Signer::new(
                ctx.get("public_key")?.to_string(),
                ctx.get("aud")?.to_string(),
            )))
        })
        .unwrap();
        reg.set_default("jwt").unwrap();
        reg
    })
}

impl AsyncFromEnv for Arc<dyn Sign> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, crate::di::EnvError> {
        validator_registry().resolve(ctx)
    }
}
