use chrono::{self, prelude::*};
use serde::{Deserialize, Serialize};
use url;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Filters>,
}

impl ListProjectsRequest {
    pub fn new() -> Self {
        Self { filters: None }
    }

    pub fn filters(mut self, filters: Filters) -> Self {
        self.filters = Some(filters);
        self
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    is_monitored: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Tags>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Attributes>,
}

impl Filters {
    pub fn new() -> Self {
        Self {
            name: None,
            origin: None,
            r#type: None,
            is_monitored: None,
            tags: None,
            attributes: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn origin(mut self, origin: String) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn r#type(mut self, r#type: String) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn is_monitored(mut self, is_monitored: bool) -> Self {
        self.is_monitored = Some(is_monitored);
        self
    }

    pub fn tags(mut self, tags: Tags) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = Some(attributes);
        self
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    includes: Vec<Tag>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    key: String,
    value: String,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    criticality: Vec<Criticality>,
    environment: Vec<String>,
    lifecycle: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Criticality {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Projects {
    pub org: Org,
    pub projects: Vec<Project>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Org {
    pub name: String,
    pub id: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    pub id: String,
    pub created: chrono::DateTime<Utc>,
    pub origin: String,
    pub r#type: String,
    pub read_only: bool,
    pub test_frequency: String,
    pub total_dependencies: Option<usize>,
    pub issue_counts_by_severity: IssueCountsBySeverity,
    pub remote_repo_url: Option<url::Url>,
    pub last_tested_date: chrono::DateTime<Utc>,
    pub importing_user: Option<User>,
    pub is_monitored: bool,
    pub owner: Option<User>,
    pub branch: Option<String>,
    pub target_reference: Option<String>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCountsBySeverity {
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub critical: usize,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: Option<String>,
    pub email: Option<String>,
}
