use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token { 
    access_token: String,
    refresh_token: String,  
    expires_at: DateTime<Utc>,
}