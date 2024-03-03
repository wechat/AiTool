use once_cell::sync::OnceCell;
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions, Error, Pool, Postgres};
use std::process;

static POSTGRES_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn init_db_pool() -> Result<(), Error> {
    // 数据库连接字符串
    let db_connection_str = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 判断数据库是否存在，不存在则创建
    if !Postgres::database_exists(&db_connection_str)
        .await
        .unwrap_or(false)
    {
        println!("创建数据库 {}", db_connection_str);
        match Postgres::create_database(&db_connection_str).await {
            Ok(_) => println!("创建数据库成功"),
            Err(err) => {
                println!("创建数据库失败: {}", err);
                process::exit(1);
            }
        }
    }

    // 连接数据库
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .unwrap();
    assert!(POSTGRES_POOL.set(pool).is_ok());
    Ok(())
}

//获取数据库
pub fn get_pool() -> Option<&'static Pool<Postgres>> {
    POSTGRES_POOL.get()
}
