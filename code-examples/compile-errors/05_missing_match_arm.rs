enum OrderStatus {
    Pending,
    Shipped { tracking_id: String },
    Cancelled { reason: String },
    Returned { reason: String },
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => String::from("waiting for payment"),
        OrderStatus::Shipped { tracking_id } => format!("shipped, tracking ID {tracking_id}"),
        OrderStatus::Cancelled { reason } => format!("cancelled: {reason}"),
    }
}

fn main() {
    let status = OrderStatus::Returned { reason: String::from("wrong size") };
    println!("{}", describe(&status));
}
