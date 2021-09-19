use std::net::SocketAddr;

use tokio::time::Instant;

use config::Config;

mod config;
mod web;

#[tokio::main]
async fn main() {
    let instant = Instant::now();

    let config = Config::extract().unwrap();

    config.init_tracing().unwrap();

    tracing::debug!("{:?}", config);

    let addr = SocketAddr::new(config.address, config.port);
    tracing::debug!("listening on {}", addr);
    let server = axum::Server::bind(&addr).serve(web::routes().into_make_service());

    tracing::info!("Started Server in {:.3?}", instant.elapsed());
    if let Err(err) = server.await {
        tracing::error!("Server error: {:?}", err)
    }
}
