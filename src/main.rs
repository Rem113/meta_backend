use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use tracing::info;
use tracing_subscriber::{filter::EnvFilter, fmt};
use warp::Filter;

mod api;
mod data;
mod domain;
mod loaders;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = fmt()
        .with_env_filter(EnvFilter::new("trace,hyper=warn,tokio_util=warn,warp=info"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not set global default subscriber");

    let docker = loaders::initialize_docker().await?;
    let docker = Arc::new(docker);

    let database = loaders::initialize_database().await?;
    let database = Arc::new(database);

    let cors = warp::cors().allow_any_origin();
    let api = warp::path("api")
        .and(api::routes(database, docker))
        .with(warp::trace::request())
        .with(cors)
        .recover(api::rejection_handler);

    let address = SocketAddr::from_str("127.0.0.1:4000").expect("Could not parse address");

    let server = warp::serve(api).run(address);

    info!("Listening on http://{address}");

    server.await;

    Ok(())
}
