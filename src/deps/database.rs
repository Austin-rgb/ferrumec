use crate::di::{AsyncFromEnv, EnvContext, EnvError};
use sqlx::{Error, Pool, Sqlite, SqlitePool};

impl From<Error> for EnvError {
    fn from(value: Error) -> Self {
        EnvError::new(value.to_string())
    }
}

impl AsyncFromEnv for Pool<Sqlite> {
    async fn from_env(ctx: &EnvContext) -> Result<Self, EnvError> {
        let url = ctx.get("DATABASE_URL")?;
        Ok(SqlitePool::connect(url).await?)
    }
}
