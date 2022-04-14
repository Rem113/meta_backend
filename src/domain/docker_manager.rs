use std::sync::Arc;

use bollard::{
    image::{BuildImageOptions, ListImagesOptions},
    models::ImageSummary,
    Docker,
};
use futures::TryStreamExt;

use crate::data::Tag;

use super::error::Error;

pub struct DockerManager {}

impl DockerManager {
    pub async fn create_image(
        docker: Arc<Docker>,
        tag: &Tag,
        image_bytes: Vec<u8>,
    ) -> Result<(), Error> {
        let mut docker_build_info = docker.build_image(
            BuildImageOptions {
                t: tag.as_meta(),
                rm: true,
                ..Default::default()
            },
            None,
            Some(image_bytes.into()),
        );

        while docker_build_info
            .try_next()
            .await
            .map_err(Error::Docker)?
            .is_some()
        {}

        Ok(())
    }

    pub async fn list_images(docker: Arc<Docker>) -> Result<Vec<ImageSummary>, Error> {
        let filters = vec![("reference", vec!["meta/*"])].into_iter().collect();

        docker
            .list_images(Some(ListImagesOptions {
                filters,
                ..Default::default()
            }))
            .await
            .map_err(Error::Docker)
    }

    pub async fn delete_image(docker: Arc<Docker>, tag: Tag) -> Result<(), Error> {
        docker
            .remove_image(&tag.as_meta(), None, None)
            .await
            .map_err(Error::Docker)?;

        Ok(())
    }
}
