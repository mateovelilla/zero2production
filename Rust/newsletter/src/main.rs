use std::net::TcpListener;
use newsletter::startup::run;
use newsletter::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?; // When we define a connection with a port in 0 then the S.O search a port enable to create the connection
    run(listener)?.await
}