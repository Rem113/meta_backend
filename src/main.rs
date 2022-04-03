use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use warp::Filter;

mod api;
mod data;
mod docker;
mod loaders;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = loaders::initialize_docker().await?;
    let docker = Arc::new(docker);

    let database = loaders::initialize_database().await?;
    let database = Arc::new(database);

    let cors = warp::cors().allow_any_origin();
    let api = warp::path("api")
        .and(api::routes(database, docker))
        .with(cors);
    let address = SocketAddr::from_str("127.0.0.1:4000").expect("Could not parse address");

    let server = warp::serve(api).run(address);

    println!("Listening on http://{address}");

    server.await;

    Ok(())
}
