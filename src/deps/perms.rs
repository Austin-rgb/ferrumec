use crate::di::{AsyncFrom, EnvContext, EnvError};

pub struct Permissions(pub String);

impl AsyncFrom<EnvContext, EnvError> for Permissions {
    async fn async_from(ctx: &EnvContext) -> Result<Self, EnvError> {
        let perms = ctx.get("PERMISSIONS")?;
        Ok(Self(perms.to_owned()))
    }
}
