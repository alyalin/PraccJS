use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub name: String,
    pub content: String,
    pub active: bool,
    pub result: String,
    pub errors: String,
}