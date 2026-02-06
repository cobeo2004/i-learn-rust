use routes::server;
use rust_api::{routes, utils};

#[tokio::main]
async fn main() {
    // Initialize environment variables and validate required ones
    utils::env::init().expect("Failed to initialize environment variables");

    tracing_subscriber::fmt::init();

    let address = format!(
        "{}:{}",
        utils::env::get("SERVER_PATH"),
        utils::env::get("SERVER_PORT")
    );

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, server::server().await.into_make_service())
        .await
        .expect("Failed to start the server. Please check the configuration and try again.");
}
