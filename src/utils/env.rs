use dotenv::dotenv;
use std::env;

/// List of all required environment variables
pub fn all() -> Vec<String> {
    vec![
        "DATABASE_URL".to_string(),
        "SERVER_PORT".to_string(),
        "SERVER_PATH".to_string(),
    ]
}

/// Initialize environment variables by loading .env file and validating required variables
pub fn init() -> Result<(), String> {
    // Load .env file
    dotenv().ok();

    // Validate all required environment variables
    let missing_vars: Vec<String> = all()
        .iter()
        .filter(|var| env::var(var).is_err())
        .cloned()
        .collect();

    if !missing_vars.is_empty() {
        return Err(format!(
            "Missing required environment variables: {}",
            missing_vars.join(", ")
        ));
    }

    Ok(())
}

/// Safely get an environment variable with a clear error message
pub fn get(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        panic!(
            "Environment variable '{}' is not set. Please check your .env file.",
            key
        )
    })
}
