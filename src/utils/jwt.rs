/// Defines JWT models.
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::core::config::Config;


const JWT_ISSUER: &str = "Authentication";
const JWT_EXPIRY_HOURS: i64 = Config::load().jwt_expiration.parse().unwrap();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    // issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user email
    pub email: String,
}

impl Claims {
    pub fn new(account_id: Uuid, email: &str) -> Claims {
        let iat = Utc::now();
        let exp = iat + Duration::hours(JWT_EXPIRY_HOURS);

        Claims {
            iss: JWT_ISSUER.to_string(),
            sub: account_id.to_string(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
            email: email.to_string(),
        }
    }
}
