use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::entity::user::Model;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
}

impl From<Model> for UserResponse {
    fn from(user: Model) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            uuid: user.uuid,
            created_at: user.created_at,
        }
    }
}
