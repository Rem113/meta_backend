use mongodb::bson::oid::ObjectId;
use warp::hyper;

use crate::{api::error_rejection::ErrorRejection, data::Repository};
use crate::data::{Execution, ExecutionDTO};

pub async fn find_by_id(
    repository: Repository,
    execution_id: ObjectId,
) -> Result<warp::reply::Json, warp::Rejection> {
    let option_execution = repository.find_by_id::<Execution>(&execution_id).await?;

    match option_execution {
        Some(execution) => Ok(warp::reply::json(&ExecutionDTO::from(execution))),
        None => Err(ErrorRejection::reject(
            "Execution not found",
            hyper::StatusCode::NOT_FOUND,
        )),
    }
}
