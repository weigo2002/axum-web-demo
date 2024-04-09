use routers::create_router;

use crate::repositories::store::Store;

mod handlers;
mod models;
mod repositories;
mod routers;

#[tokio::main]
async fn main() {
    let store = Store::new("postgres://postgres:pass1234@localhost:5432/webdev");

    let app = create_router();
    println!("Server starting...");
    let listner = tokio::net::TcpListener::bind("0.0.0.0:42001")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
