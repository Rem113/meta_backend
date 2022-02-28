use crate::model::ModelError;
use mongodb::options::ClientOptions;

pub async fn init_db() -> Result<mongodb::Client, ModelError> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = mongodb::Client::with_options(client_options)?;

    Ok(client)
}
