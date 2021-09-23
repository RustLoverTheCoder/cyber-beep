use std::net::SocketAddr;

use tokio::time::Instant;

use server::app;
use server::config::ServerConfig;

#[tokio::main]
async fn main() {
    let instant = Instant::now();

    let config = ServerConfig::extract().unwrap();

    config.init_tracing().unwrap();
    tracing::debug!("{:?}", config);

    let db = config.init_database().await.unwrap();

    let addr = SocketAddr::new(config.address, config.port);
    tracing::debug!("listening on {}", addr);

    let server = axum::Server::bind(&addr).serve(app(db).into_make_service());

    tracing::info!("Started Server in {:.3?}", instant.elapsed());
    if let Err(err) = server.await {
        tracing::error!("Server error: {:?}", err)
    }
}
