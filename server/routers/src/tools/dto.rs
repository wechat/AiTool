use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, FromRow};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Decode, Encode, FromRow)]
pub struct ToolsPayload {
    tid: Uuid,
    category_id: i32,
    sub_category_id: i32,
    title: String,
    icon: String,
    cover: String,
    link: String,
    tags: Vec<String>,
    describe: String,
    likes: i32,
    reads: i32,
    collects: i32,
    content: String,
    is_hot: bool,
    is_new: bool,
    click_list: i32,
    click_detail: i32,
}

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

#[derive(Debug, Clone, Deserialize, Serialize, Decode, Encode, FromRow)]
pub struct ToolsDetailPayload {
    tid: Uuid,
    category_id: i32,
    sub_category_id: i32,
    pub title: String,
    cover: String,
    link: String,
    tags: Vec<String>,
    pub describe: String,
    likes: i32,
    reads: i32,
    collects: i32,
    pub content: String,
    is_hot: bool,
    is_new: bool,
}
