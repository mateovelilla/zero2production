use std::net::TcpListener;
use newsletter::startup::run;
use newsletter::configuration::get_configuration;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?; // When we define a connection with a port in 0 then the S.O search a port enable to create the connection
    run(listener, connection_pool)?.await
}
