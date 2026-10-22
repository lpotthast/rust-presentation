use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    admin: bool,
}

async fn current_user() -> impl IntoResponse {
    Json(User { id: 1, name: String::from("Kevin"), admin: false })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/me", get(current_user));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
