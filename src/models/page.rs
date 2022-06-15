use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Page {
    pub page_id: i64,
    pub route: String,
    pub title: String,
    pub description: Option<String>,
    pub creation_date: i64,
}
