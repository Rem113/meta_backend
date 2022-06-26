use std::path::Path;
use std::sync::Arc;

use bollard::Docker;

use crate::{data::Tag, domain::DockerImage};

use super::Error;

pub async fn initialize_docker() -> Result<Docker, Error> {
    let docker = bollard::Docker::connect_with_local_defaults()?;

    format_image_repository(docker.clone()).await?;

    add_simulator(
        docker.clone(),
        "greeting-sim.tar.gz",
        Tag {
            name: String::from("greeting-sim"),
            version: String::from("1.0.0"),
        },
    )
    .await?;

    add_simulator(
        docker.clone(),
        "manager.tar.gz",
        Tag {
            name: String::from("manager"),
            version: String::from("1.0.0"),
        },
    )
    .await?;

    Ok(docker)
}

async fn format_image_repository(docker: Docker) -> Result<(), Error> {
    let docker = Arc::new(docker);
    let images = DockerImage::list(docker.clone()).await?;

    for image in images {
        for tag in image.repo_tags {
            match tag
                .trim_start_matches("meta/")
                .split(':')
                .collect::<Vec<_>>()
                .as_slice()
            {
                [name, version] => {
                    DockerImage::from(Tag {
                        name: String::from(*name),
                        version: String::from(*version),
                    })
                    .delete(docker.clone())
                    .await?
                }
                _ => continue,
            };
        }
    }

    Ok(())
}

async fn add_simulator(docker: Docker, path: impl AsRef<Path>, tag: Tag) -> Result<(), Error> {
    let image_file = tokio::fs::read(path)
        .await
        .map_err(|_| Error::Docker(String::from("Unexpected error while reading image file")))?;

    DockerImage::create(Arc::new(docker), tag, image_file).await?;

    Ok(())
}
