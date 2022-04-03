use crate::api::error_rejection::ErrorRejection;
use crate::data::{Image, Repository};
use bollard::image::BuildImageOptions;
use bollard::Docker;
use bytes::BufMut;
use futures::TryStreamExt;
use mongodb::bson::doc;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use warp::multipart::{FormData, Part};
use warp::Buf;

pub async fn list(repository: Repository<Image>) -> Result<warp::reply::Json, warp::Rejection> {
    let images = repository.list().await?;

    Ok(warp::reply::json(&images))
}

pub async fn create(
    repository: Repository<Image>,
    docker: Arc<Docker>,
    form_data: FormData,
) -> Result<warp::reply::Json, warp::Rejection> {
    let mut parts: HashMap<String, Part> = form_data
        .map_ok(|part| (String::from(part.name()), part))
        .try_collect()
        .await
        .map_err(|error| ErrorRejection::reject(error.to_string().as_str()))?;

    if !parts.contains_key("image") {
        return Err(ErrorRejection::reject("Missing image file"));
    }

    if !parts.contains_key("image_data") {
        return Err(ErrorRejection::reject("Missing image data"));
    }

    let image_data_part = parts.remove("image_data").unwrap();

    let image = parse_part_to_image(image_data_part).await?;

    let already_existing_image = repository
        .find(doc! {"name": image.name(), "version": image.version()})
        .await?;

    if !already_existing_image.is_empty() {
        return Err(ErrorRejection::reject("Image already exists"));
    }

    let image_file_part = parts.remove("image").unwrap();

    let image_bytes = image_file_part
        .stream()
        .try_fold(Vec::new(), |mut acc, chunk| {
            acc.put(chunk);
            async move { Ok(acc) }
        })
        .await
        .map_err(|_| ErrorRejection::reject("Failed to read image file"))?;

    let mut docker_build_info = docker.build_image(
        BuildImageOptions {
            t: format!("meta/{}:{}", image.name(), image.version()),
            rm: true,
            ..Default::default()
        },
        None,
        Some(image_bytes.into()),
    );

    while let Some(info) = docker_build_info
        .try_next()
        .await
        .map_err(|_| ErrorRejection::reject("An error has occured when building the image"))?
    {
        println!("{:?}", info);
    }

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
        .map_err(|_| ErrorRejection::reject("Couldn't parse image data"))?;

    let image = serde_json::from_str(&image_data)
        .map_err(|_| ErrorRejection::reject("Couldn't parse image data"))?;

    Ok(image)
}

pub mod rejection {
    use crate::api::error_rejection::ErrorRejection;
    use warp::body;

    pub async fn create(rejection: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
        if let Some(body::BodyDeserializeError { .. }) = rejection.find() {
            return Ok(warp::reply::with_status(
                warp::reply::json(&ErrorRejection::from("Invalid body format")),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        } else if let rejection @ Some(ErrorRejection { .. }) = rejection.find() {
            return Ok(warp::reply::with_status(
                warp::reply::json(&rejection.unwrap()),
                warp::http::StatusCode::BAD_REQUEST,
            ));
        };

        Err(rejection)
    }
}
