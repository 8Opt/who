use crate::models::auth::{AuthRegister, AuthLogin, AuthUpdate};
use crate::error::AuthError;
use uuid::Uuid;

pub async fn register(user: AuthRegister) -> Result<(), AuthError> {
    
}

pub async fn login(user: AuthLogin) -> Result<(), AuthError> {
    
}

pub async fn update(user: AuthUpdate) -> Result<(), AuthError> {
    
}

pub async fn delete(user_id: Uuid) -> Result<(), AuthError> {
    
}