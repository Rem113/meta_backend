use crate::data::EnvironmentRepository;

pub async fn list(repository: EnvironmentRepository) -> Result<warp::reply::Json, warp::Rejection> {
    let environments = repository.list().await?;

    Ok(warp::reply::json(&environments))
}
