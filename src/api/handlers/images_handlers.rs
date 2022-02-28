use crate::data::ImageRepository;
use crate::Image;

pub async fn list_handler(
    repository: ImageRepository,
) -> Result<warp::reply::Json, warp::Rejection> {
    let images = repository.list().await?;

    Ok(warp::reply::json(&images))
}

pub async fn create_handler(
    repository: ImageRepository,
    image: Image,
) -> Result<warp::reply::Json, warp::Rejection> {
    let image = repository.create(image).await?;

    Ok(warp::reply::json(&image))
}

pub mod rejection {
    use crate::api::error_rejection::ErrorRejection;
    use warp::body;

    pub async fn create_rejection_handler(
        rejection: warp::Rejection,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let reply = match rejection.find() {
            Some(body::BodyDeserializeError { .. }) => warp::reply::with_status(
                warp::reply::json(&ErrorRejection::from("Invalid body format")),
                warp::http::StatusCode::BAD_REQUEST,
            ),
            None => warp::reply::with_status(
                warp::reply::json(&ErrorRejection::from("Unexpected error")),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };

        Ok(reply)
    }
}
