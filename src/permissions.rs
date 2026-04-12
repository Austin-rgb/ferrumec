use crate::crypto::Claims;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Permission {
    pub namespace: String,
    pub name: String,
    pub value: u128,
}

impl Permission {
    pub fn check(&self, claims: Claims) -> bool {
        (self.namespace == claims.aud) && (claims.role & self.value == self.value)
    }
}
