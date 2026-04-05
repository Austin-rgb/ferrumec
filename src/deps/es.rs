use std::sync::Arc;

use event_stream::{EventStream, nats::Error, nats::NatsEventStream};

use crate::di::AsyncFromEnv;

impl AsyncFromEnv for Arc<dyn EventStream<Error = Error>> {
    async fn from_env(ctx: &crate::di::EnvContext) -> Result<Self, crate::di::EnvError> {
        let url = ctx.get("STREAM_URL")?;
        let stream = NatsEventStream::new(url).await.unwrap();
        Ok(Arc::new(stream))
    }
}
