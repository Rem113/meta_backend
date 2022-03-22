use bollard::{
    image::{BuildImageOptions, ListImagesOptions},
    Docker,
};
use futures::TryStreamExt;

use super::Error;

pub async fn initialize_docker() -> Result<Docker, Error> {
    let docker = bollard::Docker::connect_with_local_defaults()?;

    format_image_repository(&docker).await?;
    add_test_sim(&docker).await?;

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

    for image in images {
        for tag in image.repo_tags {
            docker.remove_image(tag.as_str(), None, None).await?;
        }
    }

    Ok(())
}

async fn add_test_sim(docker: &Docker) -> Result<(), Error> {
    let image_file = tokio::fs::read("test-sim.tar.gz").await.map_err(|_| {
        Error::DockerInit(String::from("Unexpected error while reading image file"))
    })?;

    let mut build_info = docker.build_image(
        BuildImageOptions {
            t: "meta/test-sim:1.0.0",
            rm: true,
            ..Default::default()
        },
        None,
        Some(image_file.into()),
    );

    while let Some(info) = build_info.try_next().await? {
        println!("{:?}", info);
    }

    Ok(())
}
