use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Object)]
pub struct ErrorResponse {
    pub message: String,
}

impl From<&(dyn std::error::Error + Send + Sync)> for ErrorResponse {
    fn from(value: &(dyn std::error::Error + Send + Sync)) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
