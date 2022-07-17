use std::sync::Arc;

use bollard::{
    image::{BuildImageOptions, ListImagesOptions, RemoveImageOptions},
    models::ImageSummary,
    Docker,
};
use futures::TryStreamExt;

use crate::data::Tag;

use super::error::DomainError;

pub struct DockerImage {
    tag: Tag,
}

impl DockerImage {
    pub fn from(tag: Tag) -> Self {
        Self { tag }
    }

    pub async fn create(
        docker: Arc<Docker>,
        tag: Tag,
        image_bytes: Vec<u8>,
    ) -> Result<Self, DomainError> {
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
            .map_err(DomainError::Docker)?
            .is_some()
        {}

        Ok(DockerImage { tag })
    }

    pub async fn list(docker: Arc<Docker>) -> Result<Vec<ImageSummary>, DomainError> {
        let filters = vec![("reference", vec!["meta/*"])].into_iter().collect();

        docker
            .list_images(Some(ListImagesOptions {
                filters,
                ..Default::default()
            }))
            .await
            .map_err(DomainError::Docker)
    }

    pub async fn delete(self, docker: Arc<Docker>) -> Result<(), DomainError> {
        docker
            .remove_image(
                &self.tag.as_meta(),
                Some(RemoveImageOptions {
                    force: true,
                    ..Default::default()
                }),
                None,
            )
            .await
            .map_err(DomainError::Docker)?;

        Ok(())
    }
}
