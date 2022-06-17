use crate::api::error_rejection::ErrorRejection;
use crate::data::{Image, Repository};
use crate::domain::DockerImage;
use bollard::Docker;
use bytes::BufMut;
use futures::TryStreamExt;
use mongodb::bson::doc;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use tracing::warn;
use warp::multipart::{FormData, Part};
use warp::{hyper, Buf};

pub async fn list(repository: Repository) -> Result<warp::reply::Json, warp::Rejection> {
    let images = repository.list::<Image>().await?;

    Ok(warp::reply::json(&images))
}

pub async fn create(
    repository: Repository,
    docker: Arc<Docker>,
    form_data: FormData,
) -> Result<warp::reply::Json, warp::Rejection> {
    let mut parts: HashMap<String, Part> = form_data
        .map_ok(|part| (String::from(part.name()), part))
        .try_collect()
        .await
        .map_err(|error| {
            warn!("{:?}", error);
            ErrorRejection::reject(
                error.to_string().as_str(),
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    if !parts.contains_key("image") {
        return Err(ErrorRejection::reject(
            "Missing image file",
            hyper::StatusCode::BAD_REQUEST,
        ));
    }

    if !parts.contains_key("image_data") {
        return Err(ErrorRejection::reject(
            "Missing image data",
            hyper::StatusCode::BAD_REQUEST,
        ));
    }

    let image_data_part = parts.remove("image_data").unwrap();

    let image = parse_part_to_image(image_data_part).await?;

    let already_existing_image = repository
        .find::<Image>(doc! {"name": &image.tag().name, "version": &image.tag().version})
        .await?;

    if !already_existing_image.is_empty() {
        return Err(ErrorRejection::reject(
            "Image already exists",
            hyper::StatusCode::CONFLICT,
        ));
    }

    let image_file_part = parts.remove("image").unwrap();

    let image_bytes = image_file_part
        .stream()
        .try_fold(Vec::new(), |mut acc, chunk| {
            acc.put(chunk);
            async move { Ok(acc) }
        })
        .await
        .map_err(|error| {
            warn!("{:?}", error);
            ErrorRejection::reject("Failed to read image file", hyper::StatusCode::BAD_REQUEST)
        })?;

    DockerImage::create_image(docker, image.tag(), image_bytes)
        .await
        .map_err(|error| {
            warn!("{:?}", error);
            ErrorRejection::reject(&error.to_string(), hyper::StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    let image = repository.create(image).await?;

    Ok(warp::reply::json(&image))
}

async fn parse_part_to_image(image_data_part: Part) -> Result<Image, warp::Rejection> {
    let image_data = image_data_part
        .stream()
        .try_fold(String::new(), |mut acc, chunk| {
            chunk
                .chunk()
                .read_to_string(&mut acc)
                .expect("Failed to read image data");
            async move { Ok(acc) }
        })
        .await
        .map_err(|error| {
            warn!("{:?}", error);
            ErrorRejection::reject("Couldn't parse image data", hyper::StatusCode::BAD_REQUEST)
        })?;

    let image = serde_json::from_str(&image_data).map_err(|error| {
        warn!("{:?}", error);
        ErrorRejection::reject("Couldn't parse image data", hyper::StatusCode::BAD_REQUEST)
    })?;

    Ok(image)
}
