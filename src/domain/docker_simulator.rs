use super::Error;

#[derive(Clone)]
pub struct DockerSimulator {
    name: String,
    port: u16,
    client: reqwest::Client,
}

impl DockerSimulator {
    pub fn new(name: String, port: u16) -> Self {
        Self {
            name,
            port,
            client: reqwest::Client::new(),
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
        command: &str,
        arguments: &serde_json::Value,
    ) -> Result<(), Error> {
        let url = format!("http://localhost:{}/command/{}", self.port, command);

        let response = self.client.post(&url).json(&arguments).send().await;

        match response {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    let response_status = response.status();
                    let response_text = response.text().await;

                    match response_text {
                        Ok(text) => Err(Error::SimulatorCommand(format!(
                            "{}: {}",
                            response_status, text
                        ))),
                        Err(error) => Err(Error::SimulatorCommand(format!(
                            "{}: {}",
                            response_status, error
                        ))),
                    }
                }
            }
            Err(err) => Err(Error::SimulatorCommand(err.to_string())),
        }
    }
}
