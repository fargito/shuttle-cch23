use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use cch23_fargito::{reindeer_cheer, reindeer_contest};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

/// Day 1
async fn recalibrate(Path(packet_ids): Path<String>) -> impl IntoResponse {
    let packet_ids: Vec<i64> = packet_ids
        .split("/")
        .map(|id| id.parse().unwrap_or(0))
        .collect();

    let mut packets_sig = 0;
    for packet_id in packet_ids {
        packets_sig = packets_sig ^ packet_id;
    }

    packets_sig.pow(3).to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/*key", get(recalibrate))
        .route("/4/strength", post(reindeer_cheer))
        .route("/4/contest", post(reindeer_contest));

    Ok(router.into())
}
