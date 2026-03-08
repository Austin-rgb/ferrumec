mod permissions;
pub use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait OnCreateHandler: Send + Sync {
    type Dto;
    async fn handle(&self, dto: Self::Dto) -> bool;
}

#[derive(Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub id: String,
    pub sku: String,
    pub quantity: u32,
}

pub use permissions::Permission;
