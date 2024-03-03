use axum::serve;
use common::log;
use db::postgresql;
use dotenv::{dotenv, var};
use routers::init::routers;
use std::{net::SocketAddr, str::FromStr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 加载环境变量
    dotenv().ok();
    //连接数据库
    postgresql::init_db_pool().await?;
    // 日志系统
    log::start_logs();
    // 加载路由
    let app = routers();
    // 启动服务
    let host_str = var("HOST").expect("HOST must be set");
    let port_str = var("PORT").expect("PORT must be set");
    let addr =
        SocketAddr::from_str(&format!("{}:{}", host_str, port_str)).expect("SocketAddr fail");
    println!("Starting server at http://{}", addr);
    let listener = TcpListener::bind(&addr).await.expect("listener fail");
    serve(listener, app).await.expect("serve start fail");
    println!("Service started successfully at http://{}", addr);
    Ok(())
}
