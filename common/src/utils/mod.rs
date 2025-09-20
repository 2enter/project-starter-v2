use project_root::get_project_root;
use sqlx::PgPool;
use std::{env, ops::Range, time::Duration};
use tokio::time::sleep;

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|e| {
        dotenvy::var(key).expect(format!("Error while finding env var `{key}`: {:?}", e).as_str())
    })
}

pub fn get_env_optional(key: &str) -> Option<String> {
    dotenvy::var(key).ok()
}

pub fn get_root_dir() -> String {
    let path = get_project_root();
    if let Ok(path) = path {
        // path.pop();
        path.to_str().unwrap().to_string()
    } else {
        get_env("APP_ROOT")
    }
}

pub async fn async_sleep(ms: i32) {
    sleep(Duration::from_millis(ms as u64)).await;
}

pub async fn async_rand_sleep(range: Range<u32>) {
    let dist = (range.end - range.start) as f32;
    sleep(Duration::from_millis(
        (rand::random::<f32>() * dist + range.start as f32) as u64,
    ))
    .await;
}

pub async fn get_pg_pool() -> PgPool {
    let database_url = get_env("DATABASE_URL").replace("?sslmode=disable", "");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
