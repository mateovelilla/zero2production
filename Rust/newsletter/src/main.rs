use newsletter::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address"); // When we define a connection with a port in 0 then the S.O search a port enable to create the connection
    run(listener)?.await
}