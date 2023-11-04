use tonic::{Request, Response, Status};

mod hello_world {
    tonic::include_proto!("hello_world");
}

use crate::hello_world::hello_service_server::{HelloService, HelloServiceServer};
use crate::hello_world::{HelloRequest, HelloResponse};

pub struct MyHelloService {}

#[tonic::async_trait]
impl HelloService for MyHelloService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let res = HelloResponse {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let hello_service = MyHelloService {};

    let mut builder = tonic::transport::Server::builder();

    let mut builder = builder.add_service(HelloServiceServer::new(hello_service));

    if cfg!(debug_assertions) {
        builder = builder.add_service(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
                    "descriptor"
                ))
                .build()?,
        );
    }

    builder.serve(addr).await?;

    Ok(())
}
