use serde::{Deserialize, Serialize};

const DEFAULT_API_URL: &str = "https://api.insaali.com";

pub fn api_url() -> String {
    std::env::var("INSAALI_API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string())
}

#[derive(Serialize)]
pub struct CreateRunRequest<'a> {
    pub simulator: &'a str,
    pub policy_ref: &'a str,
    pub max_steps: u32,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)] // Fields are parsed from the API even when not yet displayed.
pub struct Run {
    pub id: String,
    pub user_id: String,
    pub simulator: String,
    pub policy_ref: String,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Deserialize)]
pub struct LogsResponse {
    pub logs: String,
}
