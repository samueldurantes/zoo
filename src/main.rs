use axum::{routing::post, Router};
use dotenvy::dotenv;

use zoo::http;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().expect(".env file not found");

    // Building our application with a single Route
    let app = Router::new()
        // Create a charge in the Woovi API
        .route("/charge/create", post(http::charge::create_charge))
        // Receive a Webhook from Woovi, and make a new transfer to Mediator subaccount.
        .route("/transfer/create", post(http::transfer::create_transfer));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
