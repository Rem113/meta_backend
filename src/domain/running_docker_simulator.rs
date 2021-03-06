use std::sync::Arc;

use bollard::container::RemoveContainerOptions;
use bollard::Docker;
use warp::hyper;

use super::DomainError;

#[derive(Clone)]
pub struct RunningDockerSimulator {
    name: String,
    port: u16,
    client: reqwest::Client,
    docker: Arc<Docker>,
}

impl RunningDockerSimulator {
    pub fn new(name: String, port: u16, docker: Arc<Docker>) -> Self {
        Self {
            name,
            port,
            client: reqwest::Client::new(),
            docker,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn is_ready(&self) -> bool {
        self.client
            .get(&format!("http://localhost:{}/ready", self.port))
            .send()
            .await
            .is_ok()
    }

    pub async fn execute_command(
        &self,
        step: usize,
        command: &str,
        arguments: &serde_json::Value,
    ) -> Result<String, DomainError> {
        let url = format!("http://localhost:{}/command/{}", self.port, command);

        let response = self.client.post(&url).json(&arguments).send().await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response.text().await.unwrap())
                } else {
                    let response_status = response.status();
                    let response_text = response.text().await;

                    match response_text {
                        Ok(text) => Err(DomainError::SimulatorCommandFailed {
                            step,
                            message: text,
                            status: response_status,
                        }),
                        Err(_) => Err(DomainError::SimulatorCommandFailed {
                            step,
                            message: String::from("Could not get error message"),
                            status: response_status,
                        }),
                    }
                }
            }
            Err(error) => Err(DomainError::SimulatorCommandFailed {
                step,
                message: error.to_string(),
                status: hyper::StatusCode::INTERNAL_SERVER_ERROR,
            }),
        }
    }

    pub async fn remove(self) -> Result<(), DomainError> {
        self.docker
            .remove_container(
                self.name(),
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;

        Ok(())
    }
}
