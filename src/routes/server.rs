use axum::{
    Router,
    routing::{get, post},
};

use crate::routes::{root_handler::root_handler, user_handler::add_user_handler};
pub async fn server() -> Router {
    let app: Router = Router::new()
        .route("/", get(root_handler))
        .route("/user", post(add_user_handler));
    return app;
}
