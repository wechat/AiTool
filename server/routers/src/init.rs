use super::{search, tools};
use axum::{routing::MethodRouter, Router};

// 构建路由公共方法
pub fn handle_router(path: &str, method_router: MethodRouter) -> Router {
    // 设置跨域
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);
    let _path = format!("/api{}", path); // 统一api 路径
    Router::new().route(&_path, method_router).layer(cors)
}

// api 路由入口
pub fn routers() -> Router {
    auth_init_router().merge(init_router())
}

// 需要权限认证的路由
fn auth_init_router() -> Router {
    let app = Router::new();
    return app;
}

// 不需要权限认证的路由
fn init_router() -> Router {
    let app = Router::new()
        .merge(search::get_list()) // 搜索列表
        .merge(tools::get_hot()) // 热门推荐
        .merge(tools::get_new()) // 最新推出
        .merge(tools::get_category()) // 分类列表
        .merge(tools::get_tag()) // 标签列表
        .merge(tools::get_list()) // 工具列表
        .merge(tools::get_detail()); // 工具详情
    return app;
}
