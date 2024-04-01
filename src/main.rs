use std::{thread, time};

use otp_service::{OtpRequest, OtpValidationRequest, password_client::PasswordClient, validator_client::ValidatorClient};

pub mod otp_service {
    tonic::include_proto!("otp");
}


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let mut client = PasswordClient::connect("http://[::1]:8080").await?;
    let mut v_client = ValidatorClient::connect("http://[::1]:8080").await?;

    let request = tonic::Request::new(OtpRequest { username: "Joe".to_string(), timout_seconds: 30});
    let response = client.request_password(request).await?;
    let password = response.into_inner().password;
    println!("Got: '{}' from service", &password);

    let two_seconds = time::Duration::from_millis(2000);
    thread::sleep(two_seconds);

    let v_request = tonic::Request::new(OtpValidationRequest { username: "Joe".to_string(), password: password.clone()});
    let response = v_client.validate_password(v_request).await?;
    println!("Got: '{}' from service", response.into_inner().is_valid);

    println!("Waiting 30 seconds...");
    let thirty_seconds = time::Duration::from_millis(30000);
    thread::sleep(thirty_seconds);

    let v_request = tonic::Request::new(OtpValidationRequest { username: "Joe".to_string(), password});
    let response = v_client.validate_password(v_request).await?;
    println!("Got: '{}' from service", response.into_inner().is_valid);

    Ok(())
}
