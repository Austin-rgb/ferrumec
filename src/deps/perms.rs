use crate::di::{AsyncFromEnv, EnvContext, EnvError};

pub struct Permissions(pub String);

impl AsyncFromEnv for Permissions {
    async fn from_env(ctx: &EnvContext) -> Result<Self, EnvError> {
        let perms = ctx.get("PERMISSIONS")?;
        Ok(Self(perms.to_owned()))
    }
}
