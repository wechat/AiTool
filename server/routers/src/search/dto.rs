use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Decode, Encode, FromRow)]
pub struct ToolsListPayload {
    tid: Uuid,
    category_id: i32,
    sub_category_id: i32,
    pub title: String,
    icon: String,
    link: String,
    pub describe: String,
}
