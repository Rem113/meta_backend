use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Database};
use serde_json::json;

use crate::data::{Command, Environment, Image, Scenario, Simulator, Step};

use super::Error;

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
    let environment_id = initialize_environments(database).await?;
    let image_id = initialize_images(database).await?;
    let simulator_id = initialize_simulators(database, environment_id, image_id).await?;
    initalize_scenarios(database, simulator_id).await?;

    Ok(())
}

async fn initialize_environments(database: &Database) -> Result<ObjectId, Error> {
    let environments = database.collection("Environments");

    let environment = Environment::new(String::from("dev"));

    let result = environments.insert_one(environment, None).await?;

    Ok(result
        .inserted_id
        .as_object_id()
        .expect("Failed to get ObjectId for environment"))
}

async fn initialize_images(database: &Database) -> Result<ObjectId, Error> {
    let images = database.collection("Images");

    let image = Image::new(
        String::from("test-sim"),
        String::from("1.0.0"),
        vec![Command {
            name: String::from("test"),
            description: String::from("This is a test command"),
        }],
    );

    let result = images.insert_one(image, None).await?;

    Ok(result
        .inserted_id
        .as_object_id()
        .expect("Failed to get ObjectId for image"))
}

async fn initialize_simulators(
    database: &Database,
    environment_id: ObjectId,
    image_id: ObjectId,
) -> Result<ObjectId, Error> {
    let simulators = database.collection("Simulators");

    let simulator = Simulator::new(
        String::from("test-sim"),
        environment_id,
        image_id,
        HashMap::new(),
    );

    let result = simulators.insert_one(simulator, None).await?;

    Ok(result
        .inserted_id
        .as_object_id()
        .expect("Failed to get ObjectId for simulator"))
}

async fn initalize_scenarios(database: &Database, simulator_id: ObjectId) -> Result<(), Error> {
    let scenarios = database.collection("Scenarios");

    let scenario = Scenario::new(
        String::from("My scenario"),
        String::from("This is my scenario"),
        vec![Step {
            simulator_id,
            command: Command {
                name: String::from("test"),
                description: String::from("This is a test command"),
            },
            arguments: json!({}),
        }],
    );

    scenarios.insert_one(scenario, None).await?;

    Ok(())
}
