use crate::data::ScenarioRepository;

pub async fn list(repository: ScenarioRepository) -> Result<warp::reply::Json, warp::Rejection> {
    let scenarios = repository.list().await?;

    Ok(warp::reply::json(&scenarios))
}
