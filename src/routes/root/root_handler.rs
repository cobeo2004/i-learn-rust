use crate::services::root::root_service;
use axum::{Json, http::StatusCode, response::IntoResponse};

pub async fn root_handler() -> impl IntoResponse {
    let resp = root_service();
    (StatusCode::OK, Json(resp))
}
