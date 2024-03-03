use super::{
    dto::{ToolsDetailPayload, ToolsListPayload},
    servers,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use common::response::ResponseObject;
use comrak::{markdown_to_html, Options};
use db::redis::{get_key, set_key};
use serde_json::{from_str, json};
use std::collections::HashMap;
use urlencoding::encode;
use uuid::Uuid;

/**
 * 获取热门推荐
 */
pub async fn query_hot(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
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
    let storage_key = &format!("tools_query_hot_{}_{}", limit, offset);
    match get_key(storage_key) {
        Ok(value) => {
            let value: Vec<ToolsListPayload> = from_str(&value).unwrap();
            Json(ResponseObject::<Vec<ToolsListPayload>>::from_result(&value))
        }
        Err(_) => {
            let sql = format!("select tid, category_id, sub_category_id, title, icon, link, describe from tools order by is_hot desc, reads desc limit {} offset {}", limit, offset);
            let result = servers::list(&sql).await;
            match result {
                Ok(res) => {
                    match set_key(storage_key, &json!(res).to_string()) {
                        Ok(_) => println!("tools_query_hot_{}_{} set successfully", limit, offset),
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
}

/**
 * 获取最新推出
 */
pub async fn query_new(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
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
    let storage_key = &format!("tools_query_new_{}_{}", limit, offset);
    match get_key(storage_key) {
        Ok(value) => {
            let value: Vec<ToolsListPayload> = from_str(&value).unwrap();
            Json(ResponseObject::<Vec<ToolsListPayload>>::from_result(&value))
        }
        Err(_) => {
            let sql = format!("select tid, category_id, sub_category_id, title, icon, link, describe from tools order by id desc limit {} offset {}", limit, offset);
            let result = servers::list(&sql).await;
            match result {
                Ok(res) => {
                    match set_key(storage_key, &json!(res).to_string()) {
                        Ok(_) => println!("tools_query_new_{}_{} set successfully", limit, offset),
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
}

/**
 * 获取分类列表
 */
pub async fn query_category(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let category_id = params
        .get("category_id")
        .expect("category_id not found")
        .parse::<i32>()
        .unwrap_or(1);
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
    if category_id >= 0 {
        let storage_key = &format!("tools_query_category_{}_{}_{}", category_id, limit, offset);
        match get_key(storage_key) {
            Ok(value) => {
                let value: Vec<ToolsListPayload> = from_str(&value).unwrap();
                Json(ResponseObject::<Vec<ToolsListPayload>>::from_result(&value))
            }
            Err(_) => {
                let sql = format!("select tid, category_id, sub_category_id, title, icon, link, describe from tools  where category_id = {} limit {} offset {}", category_id, limit, offset);
                let result = servers::list(&sql).await;
                match result {
                    Ok(res) => {
                        match set_key(storage_key, &json!(res).to_string()) {
                            Ok(_) => println!(
                                "tools_query_category_{}_{}_{} set successfully",
                                category_id, limit, offset
                            ),
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

/**
 * 获取标签列表
 */
pub async fn query_tag(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let tag = params
        .get("tag")
        .expect("tag not found")
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
    if tag.len() > 0 {
        let storage_key = &format!("tools_query_tag_{}_{}_{}", tag, limit, offset);
        match get_key(storage_key) {
            Ok(value) => {
                let value: Vec<ToolsListPayload> = from_str(&value).unwrap();
                Json(ResponseObject::<Vec<ToolsListPayload>>::from_result(&value))
            }
            Err(_) => {
                let sql = format!("select tid, category_id, sub_category_id, tags, title, icon, link, describe from tools where '{}' ILIKE ANY (tags) limit {} offset {}", tag, limit, offset);
                let result = servers::list(&sql).await;
                match result {
                    Ok(res) => {
                        match set_key(storage_key, &json!(res).to_string()) {
                            Ok(_) => println!(
                                "tools_query_tag_{}_{}_{} set successfully",
                                tag, limit, offset
                            ),
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
        Json(ResponseObject::from_error("tag is null"))
    }
}

/**
 * 获取工具列表
 */
pub async fn query_list(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let limit = params
        .get("page_size")
        .expect("page_size not found")
        .parse::<i32>()
        .unwrap_or(24);
    let storage_key = &format!("tools_query_list_{}", limit);
    match get_key(storage_key) {
        Ok(value) => {
            let value: HashMap<i32, Vec<ToolsListPayload>> = from_str(&value).unwrap();
            Json(ResponseObject::from_result(&value))
        }
        Err(_) => {
            let mut result_map: HashMap<i32, Vec<ToolsListPayload>> = HashMap::new();
            let tid_arr = [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            ];
            for cid in tid_arr {
                let sql = format!("select tid, category_id, sub_category_id, title, icon, link, describe from tools where category_id = {} limit {} offset 0", cid, limit);
                let result: Result<Vec<ToolsListPayload>, sqlx::Error> = servers::list(&sql).await;
                match result {
                    Ok(res) => {
                        result_map.insert(cid, res);
                    }
                    Err(err) => {
                        let info = err.to_string();
                        println!("{}", info);
                    }
                }
            }
            match set_key(storage_key, &json!(result_map).to_string()) {
                Ok(_) => println!("tools_query_list_{} set successfully", limit),
                Err(e) => println!("Failed to set value: {}", e),
            }
            Json(ResponseObject::from_result(&result_map))
        }
    }
}

/**
 * 获取工具详情
 */
pub async fn query_detail(Path(tid): Path<Uuid>) -> impl IntoResponse {
    let storage_key = &format!("tools_query_detail_{}", tid);
    match get_key(storage_key) {
        Ok(value) => {
            let value: ToolsDetailPayload = from_str(&value).unwrap();
            Json(ResponseObject::<ToolsDetailPayload>::from_result(&value))
        }
        Err(_) => {
            let result = servers::detail(tid).await;
            match result {
                Ok(mut res) => {
                    let mut options = Options::default();
                    options.extension.table = true;
                    res.title = encode(&res.title).to_string();
                    res.describe = encode(&res.describe).to_string();
                    res.content = encode(&markdown_to_html(&res.content, &options)).to_string();
                    match set_key(storage_key, &json!(res).to_string()) {
                        Ok(_) => println!("tools_query_detail_{} set successfully", tid),
                        Err(e) => println!("Failed to set value: {}", e),
                    }
                    Json(ResponseObject::<ToolsDetailPayload>::from_result(&res))
                }
                Err(err) => {
                    let info = err.to_string();
                    Json(ResponseObject::<ToolsDetailPayload>::from_error(&info))
                }
            }
        }
    }
}
