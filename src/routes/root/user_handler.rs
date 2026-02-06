use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::dto::user::UserResponse;
use crate::services::user::add_user;

pub async fn add_user_handler() -> Response {
    match add_user().await {
        Ok(user) => {
            let user_response: UserResponse = user.into();
            (StatusCode::CREATED, Json(user_response)).into_response()
        }
        Err(err) => {
            let error_message = format!("Failed to create user: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": error_message
                })),
            )
                .into_response()
        }
    }
}
