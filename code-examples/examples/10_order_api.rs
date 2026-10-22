use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "status")]
enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
}

// stands in for a database query
fn find_order(id: u32) -> Option<OrderStatus> {
    match id {
        1 => Some(OrderStatus::Pending),
        2 => Some(OrderStatus::Shipped {
            tracking_id: String::from("DE123"),
        }),
        3 => Some(OrderStatus::Cancelled {
            reason: String::from("out of stock"),
        }),
        _ => None,
    }
}

async fn order_status(
    Path(id): Path<u32>,
) -> Result<Json<OrderStatus>, StatusCode> {
    let status = find_order(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(status))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/orders/{id}", get(order_status));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
