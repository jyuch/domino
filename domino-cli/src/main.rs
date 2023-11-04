mod hello_world {
    tonic::include_proto!("hello_world");
}

use crate::hello_world::hello_service_client::HelloServiceClient;
use crate::hello_world::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloServiceClient::connect("http://127.0.0.1:50051").await?;
    let request = tonic::Request::new(HelloRequest {
        name: String::from("domino"),
    });
    let response = client.say_hello(request).await?;
    println!("{}", response.into_inner().message);

    Ok(())
}
