use std::sync::Arc;

use event_stream::{EventStream, nats::NatsEventStream};

use crate::di::AsyncFromEnv;

impl AsyncFromEnv for Arc<dyn EventStream> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, crate::di::EnvError> {
        let url = ctx.get("es.url")?;
        let stream = NatsEventStream::new(url).await.unwrap();
        Ok(Arc::new(stream))
    }
}
