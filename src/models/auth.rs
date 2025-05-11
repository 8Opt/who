use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize, flatten};

// Data Transfer Object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    id: i32,
    username: String,
    password: String,
    email: String, 
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// TODO: add role



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBase {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthLogin {
    username: String,
    password: String,
}

// Route interface
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthRegister {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUpdate {
    #[serde(flatten)]
    user: AuthBase,
    email: String,
}