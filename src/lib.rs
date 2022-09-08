use async_trait::async_trait;
use reqwest;
use std::env;
use std::sync::Arc;

mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async fn list_orgs(&self) -> Result<model::Orgs, error::Error>;
}

pub struct SnykDatasource {
    http_client: Arc<reqwest::Client>,
    base_url: String,
    api_key: String,
}

impl SnykDatasource {
    pub fn new(http_client: Arc<reqwest::Client>) -> Result<Self, error::Error> {
        let api_key = env::var("SNYK_API_KEY");
        let api_key = match api_key {
            Ok(api_key) => api_key,
            Err(_) => return Err(error::Error::EnvironmentError),
        };

        Ok(Self {
            http_client,
            base_url: String::from("https://api.snyk.io"),
            api_key,
        })
    }
}

#[async_trait]
impl Datasource for SnykDatasource {
    async fn list_orgs(&self) -> Result<model::Orgs, error::Error> {
        let url = format!("{}/v1/orgs", self.base_url);
        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("token {}", self.api_key))
            .send()
            .await;

        let data = match response {
            Ok(response) => response.json::<model::Orgs>().await,
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError),
        }
    }
}
