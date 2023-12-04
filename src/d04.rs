use axum::{response::IntoResponse, Json};

/// Day 4
pub async fn reindeer_cheer(Json(body): Json<Vec<ReindeerInfo>>) -> impl IntoResponse {
    body.iter()
        .map(|reindeer| reindeer.strength)
        .sum::<i64>()
        .to_string()
}

#[derive(serde::Deserialize)]
pub struct ReindeerInfo {
    name: String,
    strength: i64,
}
