use super::{dto::ToolsListPayload, servers};
use axum::{extract::Query, response::IntoResponse, Json};
use common::response::ResponseObject;
use db::redis::{get_key, set_key};
use serde_json::{from_str, json};
use std::collections::HashMap;

/**
 * 获取工具列表
 */
pub async fn query_list(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let keyword = params
        .get("keyword")
        .expect("keyword not found")
        .parse::<String>()
        .unwrap();
    let limit = params
        .get("page_size")
        .expect("page_size not found")
        .parse::<i32>()
        .unwrap_or(10);
    let offset = params
        .get("page_num")
        .expect("page_num not found")
        .parse::<i32>()
        .unwrap_or(0);
    if keyword.len() > 0 {
        let storage_key = &format!("search_query_list_{}_{}_{}", keyword, limit, offset);
        match get_key(storage_key) {
            Ok(value) => {
                let value: Vec<ToolsListPayload> = from_str(&value).unwrap();
                Json(ResponseObject::<Vec<ToolsListPayload>>::from_result(&value))
            }
            Err(_) => {
                let sql = format!("select tid, category_id, sub_category_id, title, icon, link, describe from tools  where title ILIKE '%{}%' or describe ILIKE '%{}%' limit {} offset {}", keyword, keyword, limit, offset);
                let result = servers::list(&sql).await;
                match result {
                    Ok(res) => {
                        match set_key(storage_key, &json!(res).to_string()) {
                            Ok(_) => {
                                println!("search_query_list_{}_{} set successfully", limit, offset)
                            }
                            Err(e) => println!("Failed to set value: {}", e),
                        }
                        Json(ResponseObject::<Vec<ToolsListPayload>>::from_result(&res))
                    }
                    Err(err) => {
                        let info = err.to_string();
                        Json(ResponseObject::<Vec<ToolsListPayload>>::from_error(&info))
                    }
                }
            }
        }
    } else {
        Json(ResponseObject::from_error("keyword is null"))
    }
}
