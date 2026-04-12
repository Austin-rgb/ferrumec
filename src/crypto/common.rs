use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub as_user: String,
    pub user_id: String,
    pub aud: String,
    pub role: u128,
    pub iat: usize,
    pub jti: String, // new: unique token id (uuid)
    pub exp: usize,
}

impl Claims {
    pub fn default(as_user: String, user_id: String, aud: String) -> Self {
        let iat = Utc::now();

        Claims {
            as_user,
            user_id,
            role: 0,
            aud,
            iat: iat.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            exp: (iat.timestamp() + 300) as usize,
        }
    }

    pub fn for_admin(user_id: String) -> Self {
        let iat = Utc::now();

        Claims {
            as_user: user_id.clone(),
            user_id,
            role: u128::max_value(),
            aud: "*".to_string(),
            iat: iat.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            exp: (iat.timestamp() + 1000) as usize,
        }
    }
}
