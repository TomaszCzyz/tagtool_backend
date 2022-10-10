use tonic::{Request, Response, Status, transport::Server};

use hello_world::{HelloReply, HelloRequest};
use hello_world::greeter_server::{Greeter, GreeterServer};
use tag_service::{CreateTagsRequest, CreateTagsReply};
use tag_service::tag_service_server::{TagService, TagServiceServer};

// use tag_search_service::tag_search_service_server::*;

// pub mod hello_world {
//     tonic::include_proto!("hello_world");
// }

pub mod hello_world {
    tonic::include_proto!("hello_world");
}

pub mod tag_search_service {
    tonic::include_proto!("tag_search_service");
}

pub mod tag_service {
    tonic::include_proto!("tag_service");
}


#[derive(Default)]
pub struct MyGreeter {}

#[derive(Default)]
pub struct MyTagService {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl TagService for MyTagService {
    async fn create_tags(
        &self,
        request: Request<CreateTagsRequest>,
    ) -> Result<Response<CreateTagsReply>, Status> {
        println!("Tag - Got a request from {:?}", request.remote_addr());

        let reply = tag_service::CreateTagsReply {
            message: format!("Tag - Hello!"),
        };
        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();
    let tag_service = MyTagService::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(TagServiceServer::new(tag_service))
        .serve(addr)
        .await?;

    Ok(())
}