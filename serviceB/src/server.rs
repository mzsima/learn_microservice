use tonic::{transport::Server, Request, Response, Status};

use base64encoder::base64_encoder_server::{Base64Encoder, Base64EncoderServer};
use base64encoder::{Base64EncodeReply, Base64EncodeRequest};

use base64::encode;

pub mod base64encoder {
    tonic::include_proto!("base64encoder");
}

#[derive(Debug, Default)]
pub struct MyEncoder {}

#[tonic::async_trait]
impl Base64Encoder for MyEncoder {
    async fn base64_encode(
        &self,
        request: Request<Base64EncodeRequest>,
    ) -> Result<Response<Base64EncodeReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = base64encoder::Base64EncodeReply {
            message: encode(request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let base64_encoder = MyEncoder::default();

    Server::builder()
        .add_service(Base64EncoderServer::new(base64_encoder))
        .serve(addr)
        .await?;

    Ok(())
}
