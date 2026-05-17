use crate::di::{AsyncFrom, EnvContext, EnvError};
use sqlx::{Error, Pool, Sqlite, SqlitePool};

impl From<Error> for EnvError {
    fn from(value: Error) -> Self {
        EnvError::new(value.to_string())
    }
}

impl AsyncFrom<EnvContext, EnvError> for Pool<Sqlite> {
    async fn async_from(ctx: &EnvContext) -> Result<Self, EnvError> {
        let url = ctx.get("DATABASE_URL")?;
        Ok(SqlitePool::connect(url).await?)
    }
}
