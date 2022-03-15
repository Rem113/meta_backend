use crate::{
    api::error_rejection::ErrorRejection,
    data::{Simulator, SimulatorRepository},
};

pub async fn list(repository: SimulatorRepository) -> Result<warp::reply::Json, warp::Rejection> {
    let simulators = repository.list().await?;

    Ok(warp::reply::json(&simulators))
}

pub async fn create(
    repository: SimulatorRepository,
    simulator: Simulator,
) -> Result<warp::reply::Json, warp::Rejection> {
    let simulators_for_environment = repository
        .find_by_environment(simulator.environment_id())
        .await?;

    for other in simulators_for_environment {
        if other.name() == simulator.name() {
            return Err(ErrorRejection::reject(
                "A simulator with the same name exists in this environment",
            ));
        }
    }

    let simulator = repository.create(simulator).await?;

    Ok(warp::reply::json(&simulator))
}