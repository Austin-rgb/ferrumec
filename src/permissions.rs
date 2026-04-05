use libsigners::Claims;
use serde::Serialize;

#[derive(Clone, Serialize)]
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
