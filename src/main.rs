use otp_service::{OtpRequest, OtpValidationRequest, password_client::PasswordClient, validator_client::ValidatorClient};

pub mod otp_service {
    tonic::include_proto!("otp");
}


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let mut client = PasswordClient::connect("http://[::1]:8080").await?;
    let mut v_client = ValidatorClient::connect("http://[::1]:8080").await?;
    let request = tonic::Request::new(OtpRequest { timout_seconds: 30});
    let v_request = tonic::Request::new(OtpValidationRequest { password: "12345".parse().unwrap()});

    let response = client.request_password(request).await?;
    println!("Got: '{}' from service", response.into_inner().password);

    let response = v_client.validate_password(v_request).await?;
    println!("Got: '{}' from service", response.into_inner().is_valid);
    Ok(())
}
