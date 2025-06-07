use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub score: Option<u32>,
}

impl User {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            score: None,
        }
    }

    pub fn with_score(&self, score: u32) -> Self {
        let mut updated = self.clone();
        updated.score = Some(score);
        updated
    }
}