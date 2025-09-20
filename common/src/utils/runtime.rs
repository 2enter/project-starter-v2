use crate::utils::env::get_env;
use sqlx::PgPool;
use std::{ops::Range, time::Duration};
use tokio::time::sleep;

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
