mod route;
mod controller;

use config;
use config::config::Config;

#[tokio::main]
async fn main() {

    let app = route::route::app();
    let config: Config = config::config::read_config();
    let bind_uri = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(bind_uri)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
