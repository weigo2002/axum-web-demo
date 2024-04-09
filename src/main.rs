use routers::create_router;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::repositories::store::Store;

mod handlers;
mod models;
mod repositories;
mod routers;

#[tokio::main]
async fn main() {
    let store = Store::new("postgres://postgres:pass1234@localhost:5432/rustwebdev").await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migrate");

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let app = create_router();
    println!("Server starting...");
    let listner = tokio::net::TcpListener::bind("0.0.0.0:42001")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
