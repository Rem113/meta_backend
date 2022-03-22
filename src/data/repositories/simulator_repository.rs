use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection, Database,
};

use crate::data::{Error, Simulator};

pub struct SimulatorRepository {
    simulators: Collection<Simulator>,
}

impl SimulatorRepository {
    pub fn new(database: &Database) -> Self {
        Self {
            simulators: database.collection("Simulators"),
        }
    }

    pub async fn list(&self) -> Result<Vec<Simulator>, Error> {
        let cursor = self.simulators.find(None, None).await?;

        cursor.try_collect().await.map_err(Error::from)
    }

    pub async fn create(&self, simulator: Simulator) -> Result<Simulator, Error> {
        let result = self.simulators.insert_one(&simulator, None).await?;

        let inserted_id = result.inserted_id.as_object_id().expect("Invalid ObjectID");

        Ok(simulator.with_id(inserted_id))
    }

    pub async fn find_by_environment(
        &self,
        environment_id: &ObjectId,
    ) -> Result<Vec<Simulator>, Error> {
        let filter = doc! {"environment_id": environment_id};

        let cursor = self.simulators.find(Some(filter), None).await?;

        cursor.try_collect().await.map_err(Error::from)
    }
}
