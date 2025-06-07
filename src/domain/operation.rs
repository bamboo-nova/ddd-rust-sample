use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Operation {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub command: String,
}