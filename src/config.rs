use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// "Owner/Repo" -> "Git URL" map
    pub repos: HashMap<String, String>,
}
