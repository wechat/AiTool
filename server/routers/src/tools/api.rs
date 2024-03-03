use super::handler::{query_category, query_detail, query_hot, query_list, query_new, query_tag};
use crate::init::handle_router;
use axum::{routing::get, Router};

const ROUTER_TOOLS: &str = "/tools";

/**
 * 获取热门推荐
 */
pub fn get_hot() -> Router {
    let path = format!("{}/hot", ROUTER_TOOLS);
    handle_router(&path, get(query_hot))
}

/**
 * 获取最新推出
 */
pub fn get_new() -> Router {
    let path = format!("{}/new", ROUTER_TOOLS);
    handle_router(&path, get(query_new))
}

/**
 * 分类列表
 */
pub fn get_category() -> Router {
    let path = format!("{}/category", ROUTER_TOOLS);
    handle_router(&path, get(query_category))
}

/**
 * 标签列表
 */
pub fn get_tag() -> Router {
    let path = format!("{}/tags", ROUTER_TOOLS);
    handle_router(&path, get(query_tag))
}

/**
 * 获取工具列表
 */
pub fn get_list() -> Router {
    let path = format!("{}/list", ROUTER_TOOLS);
    handle_router(&path, get(query_list))
}

/**
 * 获取工具详情
 */
pub fn get_detail() -> Router {
    let path = format!("{}/detail/:tid", ROUTER_TOOLS);
    handle_router(&path, get(query_detail))
}
