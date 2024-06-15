// Integration tests
use std::net::TcpListener;
fn spawn_app() -> String{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address"); // When we define a connection with a port in 0 then the S.O search a port enable to create the connection
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind adress"); // When we define a connection with a port in 0 then the S.O search a port enable to create the connection
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[tokio::test]
async fn health_check_works(){
    let address = spawn_app();
    let client = reqwest::Client::new();

    //Act
    let response = client.get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
