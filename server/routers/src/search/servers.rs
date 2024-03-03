use super::dto::ToolsListPayload;
use db::postgresql;
use sqlx::{self, Error};
use urlencoding::encode;

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
