use sqlx::PgPool;
use std::net::TcpListener;

use members::configuration::get_configuration;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_configuration().expect("Failed to load config");
    let connection = PgPool::connect(
            &configuration.database.connection_string()
        )
        .await
        .expect("failed to connect to postgres");
    let server = members::startup::run(listener, connection.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp{
        address,
        db_pool: connection
    }
}