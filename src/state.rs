use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct State {
    pub last_accessed_thought_id: Option<String>,
}
