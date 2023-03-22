use tonic::{transport::Server, Request, Response, Status};

// import grpc service
use user::user_service_server::{UserService, UserServiceServer};
// import grpc messages
use user::{HelloResponse, HelloRequest};

// define the module and use the macro to import everything into it for the above code
// this will become the namespaced used everywhere else.
pub mod user {
    tonic::include_proto!("user"); // The string specified here must match the proto package name
}

// struct to contains all the methods aka our endpoints
#[derive(Debug, Default)]
pub struct UserMethodsWrapper {}


// implement our service on our server struct
#[tonic::async_trait]
impl UserService for UserMethodsWrapper {
    // accept a request of type HelloRequest, respond with a result with generic of our hello response type
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        // from our defined module up top on line 9
        let reply = user::HelloResponse {
            // We must use .into_inner() as the fields of gRPC requests and responses are private
            message: String::from("Hello Sam!")
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build the actual service
    let addr = "[::1]:50051".parse()?;
    // init our UserMethodsWrapper struct
    let user_methods = UserMethodsWrapper::default();

    Server::builder()
        .add_service(UserServiceServer::new(user_methods))
        .serve(addr)
        .await?;

    Ok(())
}

// brew install grpcurl
// packagename.ServiceName/MethodName
// grpcurl -plaintext -import-path ./proto -proto users.proto -d '{"name": "Sam"}' '[::]:50051' user.UserService/SayHello
