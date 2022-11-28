use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli{
    #[command(subcommand)]
    mode: Option<Modes>,
}

#[derive(Subcommand, Debug)]
enum Modes {
    Server {
        #[arg(short,long,default_value_t=9000)]
        port: i32,
    },
}


use tonic::{transport::Server, Request, Response, Status};
use server::greeter_server::{Greeter, GreeterServer};
use server::{HelloRequest, HelloReply};

pub mod server {
    tonic::include_proto!("server");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = server::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match &cli.mode {
        Some(Modes::Server { port }) => {
            let addr = "[::1]:50051".parse().unwrap();
            let greeter = MyGreeter::default();
        
            println!("GreeterServer listening on {}", addr);
        
            Server::builder()
                .add_service(GreeterServer::new(greeter))
                .serve(addr)
                .await?;
        
            Ok(())
        }
        None => {Ok(())}
    }
}