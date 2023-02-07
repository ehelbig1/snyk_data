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
    created: chrono::DateTime<Utc>,
    origin: String,
    r#type: String,
    read_only: bool,
    test_frequency: String,
    total_dependencies: Option<usize>,
    issue_counts_by_severity: IssueCountsBySeverity,
    remote_repo_url: Option<url::Url>,
    last_tested_date: chrono::DateTime<Utc>,
    importing_user: Option<User>,
    is_monitored: bool,
    owner: Option<User>,
    branch: Option<String>,
    target_reference: Option<String>,
    tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCountsBySeverity {
    low: usize,
    medium: usize,
    high: usize,
    critical: usize,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    name: String,
    username: Option<String>,
    email: Option<String>,
}
