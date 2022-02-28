use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use warp::Filter;

use crate::model::{initialize_database, Command, Image, Simulator};

mod api;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = initialize_database().await?;
    let database = Arc::new(database);

    let api = warp::path("api").and(api::routes(database));
    let address = SocketAddr::from_str("127.0.0.1:3000").expect("Could not parse address");

    let server = warp::serve(api).run(address);

    println!("Listening on http://{address}");

    server.await;

    Ok(())
}
