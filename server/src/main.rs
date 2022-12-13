#![allow(unused)]

use tonic::{transport::Server, Request, Response, Status};

use hello::hello_server::{Hello, HelloServer};
use hello::{HelloRequest, HelloResponse};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn send_hello(
        &self, 
        request: Request<HelloRequest>
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        let reply = HelloResponse { 
            status: true, 
            message: format!("Hello there {}", req.name) 
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = HelloService::default();

    Server::builder()
        .add_service(HelloServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}