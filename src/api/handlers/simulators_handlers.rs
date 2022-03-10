use crate::data::SimulatorRepository;

pub async fn list(repository: SimulatorRepository) -> Result<warp::reply::Json, warp::Rejection> {
    let simulators = repository.list().await?;

    Ok(warp::reply::json(&simulators))
}
