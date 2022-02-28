use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Database};

use crate::model::Error;
use crate::{Command, Image, Simulator};

pub async fn initialize_database() -> Result<Database, Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("meta");

    format_database(&database).await?;
    populate_database(&database).await?;

    Ok(database)
}

async fn format_database(database: &Database) -> Result<(), Error> {
    database.drop(None).await?;

    Ok(())
}

async fn populate_database(database: &Database) -> Result<(), Error> {
    let image_id = initialize_images(database).await?;
    initialize_simulators(database, image_id).await?;

    Ok(())
}

async fn initialize_images(database: &Database) -> Result<ObjectId, Error> {
    let images = database.collection("Images");

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

    let result = images.insert_one(image, None).await?;

    Ok(result
        .inserted_id
        .as_object_id()
        .expect("Failed to get ObjectId for image"))
}

async fn initialize_simulators(database: &Database, image_id: ObjectId) -> Result<(), Error> {
    let simulators = database.collection("Simulators");

    let simulator = Simulator::new(
        String::from("kafka-resolver"),
        image_id,
        HashMap::from([(
            String::from("KAFKA_HOST"),
            String::from("kafka://host:1234"),
        )]),
    );

    simulators.insert_one(simulator, None).await?;

    Ok(())
}
