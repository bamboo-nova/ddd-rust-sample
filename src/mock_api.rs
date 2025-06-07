use axum::{routing::{get, post}, Json, Router};
use axum_server::Server;
use serde_json::Value;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // /operations で GET & POST
    let app = Router::new()
        .route("/operations", get(list_ops))
        .route("/operations/{id}/result", post(receive_result));

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("mock_api listening on {addr}");
    Server::bind(addr).serve(app.into_make_service()).await.unwrap();
}

async fn list_ops() -> Json<Vec<Value>> {
    Json(vec![
        serde_json::json!({
            "id": "stu-001",
            "user_id": "1751057",
            "status": "pending",
            "command": "ping"
        }),
        serde_json::json!({
            "id": "stu-002",
            "user_id": "1751057",
            "status": "done",
            "command": "noop"
        }),
    ])
}

async fn receive_result(
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(body): Json<Value>,
) {
    println!("★ result for {id}: {body}");
}
