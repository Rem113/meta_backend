use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Database};
use serde_json::json;

use crate::data::{Command, Environment, Image, Scenario, Simulator, Step, Tag};

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
    let (greeting_sim_id, manager_id) = initialize_images(database).await?;
    initialize_greeting_simulator(database, environment_id, greeting_sim_id).await?;
    initialize_manager(database, environment_id, manager_id).await?;
    initialize_scenarios(database, greeting_sim_id, manager_id).await?;

    Ok(())
}

async fn initialize_environments(database: &Database) -> Result<ObjectId, Error> {
    let environments = database.collection("Environments");

    let environment = Environment::new(
        String::from("dev"),
        String::from("Runs in the dev environment"),
    );

    let result = environments.insert_one(environment, None).await?;

    Ok(result
        .inserted_id
        .as_object_id()
        .expect("Failed to get ObjectId for environment"))
}

async fn initialize_images(database: &Database) -> Result<(ObjectId, ObjectId), Error> {
    let images = database.collection("Images");

    let greeting_sim_image = Image::new(
        Tag {
            name: String::from("greeting-sim"),
            version: String::from("1.0.0"),
        },
        vec![Command {
            name: String::from("Greet"),
            description: String::from("This command takes a name as a parameter, and returns a greeting for the specified name"),
            path: String::from("greet"),
        }],
    );

    let manager_image = Image::new(
        Tag {
            name: String::from("manager"),
            version: String::from("1.0.0"),
        },
        vec![Command {
            name: String::from("Sleep"),
            description: String::from("This command takes a duration as a parameter, and sleeps for the specified duration"),
            path: String::from("sleep"),
        }]
    );

    let greeting_sim_insert_result = images.insert_one(greeting_sim_image, None).await?;
    let manager_insert_result = images.insert_one(manager_image, None).await?;

    Ok((
        greeting_sim_insert_result
            .inserted_id
            .as_object_id()
            .expect("Failed to get ObjectId for greeting-sim image"),
        manager_insert_result
            .inserted_id
            .as_object_id()
            .expect("Failed to get ObjectId for manager image"),
    ))
}

async fn initialize_greeting_simulator(
    database: &Database,
    environment_id: ObjectId,
    image_id: ObjectId,
) -> Result<(), Error> {
    let simulators = database.collection("Simulators");

    let simulator = Simulator::new(
        String::from("greeting-sim"),
        environment_id,
        image_id,
        HashMap::from([(String::from("GREETING"), String::from("Hey"))]),
    );

    simulators.insert_one(simulator, None).await?;

    Ok(())
}

async fn initialize_manager(
    database: &Database,
    environment_id: ObjectId,
    image_id: ObjectId,
) -> Result<(), Error> {
    let simulators = database.collection("Simulators");

    let simulator = Simulator::new(
        String::from("manager"),
        environment_id,
        image_id,
        HashMap::new(),
    );

    simulators.insert_one(simulator, None).await?;

    Ok(())
}

async fn initialize_scenarios(
    database: &Database,
    greeting_sim_image_id: ObjectId,
    manager_image_id: ObjectId,
) -> Result<(), Error> {
    let scenarios = database.collection("Scenarios");

    let scenario = Scenario::new(
        String::from("Steps are run until the first failure"),
        String::from(
            "This scenario checks that steps after the first failing step are not run by Meta",
        ),
        vec![
            Step {
                image_id: greeting_sim_image_id,
                command: Command {
                    name: String::from("Greet"),
                    description: String::from("Checks that the greeting is correct"),
                    path: String::from("greet"),
                },
                arguments: json!({ "name": "Rem113" }),
            },
            Step {
                image_id: manager_image_id,
                command: Command {
                    name: String::from("Sleep"),
                    description: String::from("Waits for 5 seconds"),
                    path: String::from("sleep"),
                },
                arguments: json!({ "duration": 5000 }),
            },
            Step {
                image_id: greeting_sim_image_id,
                command: Command {
                    name: String::from("Greet"),
                    description: String::from(
                        "This command is missing the name parameter, so it should fail",
                    ),
                    path: String::from("greet"),
                },
                arguments: json!({}),
            },
            Step {
                image_id: greeting_sim_image_id,
                command: Command {
                    name: String::from("Greet"),
                    description: String::from(
                        "Because the last step should fail, this command should never be run",
                    ),
                    path: String::from("greet"),
                },
                arguments: json!({ "name": "Ninja" }),
            },
        ],
    );

    scenarios.insert_one(scenario, None).await?;

    Ok(())
}
