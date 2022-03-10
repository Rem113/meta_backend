use bollard::{image::ListImagesOptions, Docker};

use super::Error;

pub async fn initialize_docker() -> Result<Docker, Error> {
    let docker = bollard::Docker::connect_with_local_defaults()?;

    format_image_repository(&docker).await?;

    Ok(docker)
}

async fn format_image_repository(docker: &Docker) -> Result<(), Error> {
    let filters = vec![("reference", vec!["meta/*"])].into_iter().collect();

    let images = docker
        .list_images(Some(ListImagesOptions {
            filters,
            ..Default::default()
        }))
        .await?;

    for mut image in images {
        let tag = image.repo_tags.pop().ok_or(Error::DockerInit(String::from(
            "Unexpected error while cleaning Docker image repository",
        )))?;

        docker.remove_image(tag.as_str(), None, None).await?;
    }

    Ok(())
}
