use once_cell::sync::Lazy;
use redis::{Commands, RedisResult};
use std::sync::Mutex;

// 定义一个全局的Redis连接
static REDIS_CONN: Lazy<Mutex<redis::Connection>> = Lazy::new(|| {
    // 数据库连接字符串
    let redis_connection_str = dotenv::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let client = redis::Client::open(redis_connection_str).expect("Redis client creation failed");
    let conn = client.get_connection().expect("Failed to connect to Redis");
    Mutex::new(conn)
});

pub fn set_key(key: &str, value: &str) -> RedisResult<()> {
    let mut conn = REDIS_CONN.lock().unwrap();
    conn.set_ex(key, value, 60 * 60 * 1)
}

pub fn get_key(key: &str) -> RedisResult<String> {
    let mut conn = REDIS_CONN.lock().unwrap();
    conn.get(key)
}
