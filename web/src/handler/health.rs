use axum::Json;

pub async fn get() -> Json<bool> {
    true.into()
}
