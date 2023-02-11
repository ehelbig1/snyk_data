use async_trait::async_trait;
use reqwest;
use serde_json;

pub mod error;
pub mod model;

#[async_trait]
pub trait Datasource {
    async fn list_orgs(&self) -> Result<model::org::Orgs, error::Error>;

    // This only includes sca, container, and iac issues
    async fn list_aggregated_sca_container_iac_issues(
        &self,
        org_id: &str,
        project_id: &str,
    ) -> Result<model::issue::Issues, error::Error>;

    async fn list_sast_issues(&self, org_id: &str, project_id: &str, properties: &model::issue_v3::SnykCodeIssuesRequest) -> Result<model::issue_v3::Response, error::Error>;
    async fn list_projects(&self, org_id: &str, properties: &model::projects::ListProjectsRequest) -> Result<model::projects::Projects, error::Error>;
    async fn next(&self, path: &str) -> Result<model::issue_v3::Response, error::Error>;
}

pub struct SnykDatasource<'a> {
    http_client: &'a reqwest::Client,
    base_url: String,
    api_key: &'a str,
}

impl<'a> SnykDatasource<'a> {
    pub fn new(http_client: &'a reqwest::Client, api_key: &'a str) -> Self {
        Self {
            http_client,
            base_url: String::from("https://api.snyk.io"),
            api_key,
        }
    }
}

#[async_trait]
impl<'a> Datasource for SnykDatasource<'a> {
    async fn list_orgs(&self) -> Result<model::org::Orgs, error::Error> {
        let url = format!("{}/v1/orgs", self.base_url);
        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("token {}", self.api_key))
            .send()
            .await;

        let data = match response {
            Ok(response) => response.json::<model::org::Orgs>().await,
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError),
        }
    }

    async fn list_aggregated_sca_container_iac_issues(
        &self,
        org_id: &str,
        project_id: &str,
    ) -> Result<model::issue::Issues, error::Error> {
        let url = format!(
            "{}/api/v1/org/{}/project/{}/aggregated-issues",
            self.base_url, org_id, project_id
        );
        let body = model::issue::AggregatedIssuesRequest::new();

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("token {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await;

        let data = match response {
            Ok(response) => response.json::<model::issue::Issues>().await,
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError),
        }
    }

    async fn list_sast_issues(&self, org_id: &str, project_id: &str, properties: &model::issue_v3::SnykCodeIssuesRequest) -> Result<model::issue_v3::Response, error::Error> {
        let url = format!("{}/rest/orgs/{}/issues", self.base_url, org_id);

        let response = self.http_client
            .get(&url)
            .header("Authorization", format!("token {}", self.api_key))
            .header("Content-Type", "application/vnd.api+json")
            .query(&[("project_id", project_id), ("version", &properties.version)])
            .send()
            .await;

        let data = match response {
            Ok(response) => response.json::<model::issue_v3::Response>().await,
            Err(_) => return Err(error::Error::RequestError)
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError)
        }
    }

    async fn list_projects(&self, org_id: &str, properties: &model::projects::ListProjectsRequest) -> Result<model::projects::Projects, error::Error> {
        let url = format!("{}/api/v1/org/{}/projects", self.base_url, org_id);

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("token {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&properties).unwrap())
            .send()
            .await;

        let data = match response {
            Ok(response) => response.json::<model::projects::Projects>().await,
            Err(_) => return Err(error::Error::RequestError),
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError),
        }
    }

    async fn next(&self, path: &str) -> Result<model::issue_v3::Response, error::Error> {
        let url = format!("{}/rest{}", self.base_url, path);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", format!("token {}", self.api_key))
            .header("Content-Type", "application/vnd.api+json")
            .send()
            .await;

        let data = match response {
            Ok(response) => response.json::<model::issue_v3::Response>().await,
            Err(_) => return Err(error::Error::RequestError)
        };

        match data {
            Ok(data) => Ok(data),
            Err(_) => Err(error::Error::ParseError)
        }
    }
}
