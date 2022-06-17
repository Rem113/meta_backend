use std::sync::Arc;

use bollard::Docker;

use crate::{data::Tag, domain::DockerImage};

use super::Error;

pub async fn initialize_docker() -> Result<Docker, Error> {
    let docker = bollard::Docker::connect_with_local_defaults()?;

    format_image_repository(docker.clone()).await?;
    add_test_sim(docker.clone()).await?;

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

async fn add_test_sim(docker: Docker) -> Result<(), Error> {
    let image_file = tokio::fs::read("test-sim.tar.gz")
        .await
        .map_err(|_| Error::Docker(String::from("Unexpected error while reading image file")))?;

    DockerImage::create(
        Arc::new(docker),
        Tag {
            name: String::from("test-sim"),
            version: String::from("1.0.0"),
        },
        image_file,
    )
    .await?;

    Ok(())
}
