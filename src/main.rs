use chrono::{DateTime, Utc};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello"); // Matches proto package name
}

use hello::hello_service_server::{HelloService, HelloServiceServer};
use hello::{HelloRequest, HelloResponse};
use vesture::configuration::get_configuration;

#[derive(Debug, Default)]
pub struct MyHello;

#[tonic::async_trait]
impl HelloService for MyHello {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let name = request.into_inner().name;
        let reply = HelloResponse {
            message: format!("Hello, {}!", name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_configuration().expect("Failed to read configuration.");

    let pool = sqlx::PgPool::connect_with(config.database.connect_options())
        .await
        .expect("Failed to connect to database.");

    let row: (DateTime<Utc>,) = sqlx::query_as("SELECT now()")
        .fetch_one(&pool)
        .await
        .expect("Failed to run test query");

    println!("Database time: {}", row.0);

    let addr = "[::1]:50051".parse()?;
    let svc = MyHello::default();

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(HelloServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
