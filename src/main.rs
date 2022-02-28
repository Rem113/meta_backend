use std::collections::HashMap;
use std::sync::Arc;

use crate::model::{init_db, Command, Image, Simulator};
use mongodb::bson::doc;
use warp::Filter;

mod api;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = init_db().await?;
    let db = client.database("meta");
    let db = Arc::new(db);

    let images = db.collection::<Image>("Images");

    let image = Image::new(
        String::from("kafka-resolver"),
        String::from("1.0.0"),
        vec![
            Command::new(
                String::from("listen_to_topic"),
                String::from(
                    "Starts listening to a topic. Get the messages with 'get_messages_from_topic'",
                ),
            ),
            Command::new(
                String::from("get_messages_from_topic"),
                String::from("Gets the messages sent since started listening"),
            ),
        ],
    );

    images.insert_one(image, None).await?;

    let image_query = doc! { "name": "kafka-resolver" };
    let image = images
        .find_one(image_query, None)
        .await?
        .expect("Image not found");

    let simulator = Simulator::new(
        String::from("kafka-resolver"),
        image.id().unwrap().clone(),
        HashMap::from([(
            String::from("KAFKA_HOST"),
            String::from("kafka://host:1234"),
        )]),
    );

    let simulators = db.collection::<Simulator>("Simulators");

    let inserted_simulator = simulators.insert_one(simulator, None).await?;

    let simulator_query = doc! { "_id": inserted_simulator.inserted_id };

    let simulator = simulators
        .find_one(simulator_query, None)
        .await?
        .expect("Simulator not found");

    println!("{simulator:#?}");

    let image_query = doc! { "_id": simulator.image_id() };

    let image = images
        .find_one(image_query, None)
        .await?
        .expect("Image not found");

    println!("{image:#?}");

    warp::serve(warp::path("api").and(api::images_routes(db)))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
