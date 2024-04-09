use routers::create_router;

mod handlers;
mod routers;

#[tokio::main]
async fn main() {
    let app = create_router();
    println!("Server starting...");
    let listner = tokio::net::TcpListener::bind("0.0.0.0:42001")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
