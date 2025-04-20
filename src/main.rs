use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new();

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", address);

    axum::serve(tokio::net::TcpListener::bind(address).await.unwrap(), app)
        .await
        .unwrap();
}
