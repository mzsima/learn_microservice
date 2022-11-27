use base64encoder::base64_encoder_client::Base64EncoderClient;
use base64encoder::Base64EncodeRequest;

pub mod base64encoder {
    tonic::include_proto!("base64encoder");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Base64EncoderClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Base64EncodeRequest {
        message: "mzsima".to_string(),
    });

    let response = client.base64_encode(request).await?;

    println!("RESPONSE{:?}", response);

    Ok(())
}
