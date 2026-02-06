use serde::Serialize;

#[derive(Serialize)]
pub struct RootResponse {
    pub message: String,
}

pub fn root_service() -> RootResponse {
    RootResponse {
        message: "Hello, world!".to_string(),
    }
}
