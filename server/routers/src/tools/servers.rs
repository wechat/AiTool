use super::dto::{ToolsDetailPayload, ToolsListPayload};
use db::postgresql;
use sqlx::{self, Error};
use urlencoding::encode;
use uuid::Uuid;

/**
 * 获取工具列表
 */
pub async fn list(sql: &str) -> Result<Vec<ToolsListPayload>, Error> {
    let pool = postgresql::get_pool().unwrap();
    let mut res = sqlx::query_as::<_, ToolsListPayload>(sql)
        .fetch_all(pool)
        .await?;
    for item in &mut res {
        item.title = encode(&item.title).to_string();
        item.describe = encode(&item.describe).to_string();
    }
    Ok(res)
}

/**
 * 获取工具详情
 */
pub async fn detail(tid: Uuid) -> Result<ToolsDetailPayload, Error> {
    let pool = postgresql::get_pool().unwrap();
    let sql =
        "select tid, category_id, sub_category_id, title, cover, link, tags, describe, likes, reads, collects, content, is_hot, is_new from tools where tid = $1";
    let res = sqlx::query_as::<_, ToolsDetailPayload>(sql)
        .bind(tid)
        .fetch_one(pool)
        .await?;
    Ok(res)
}
