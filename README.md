Just a back up of an example of using tonic for creating a gRPC server in rust

Usage
```
cargo run --release
```
send request to imitate client request with grpcurl or postman
```
grpcurl -plaintext -import-path ./proto -proto users.proto -d '{"name": "Sam"}' '[::]:50051' user.UserService/SayHello
```