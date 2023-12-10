mod route;
mod controller;

use api::model::user_model::User;
use config;
use config::config::Config;

#[tokio::main]
async fn main() {
    let _u = User {
        id: 0,
        user_name: None,
        age: None,
    };
    let app = route::route::app();
    db::mysql::mysql_pool().await;
    let config: Config = config::config::read_config();
    let bind_uri = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(bind_uri)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
