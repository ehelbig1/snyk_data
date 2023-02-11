use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub struct SnykCodeIssuesRequest {
    pub severity: Option<Severity>,
    pub r#type: Option<String>,
    pub starting_after: Option<String>,
    pub ending_before: Option<String>,
    pub limit: Option<usize>,
    pub version: String
}

impl SnykCodeIssuesRequest {
    pub fn new() -> Self {
        Self {
            severity: None,
            r#type: None,
            starting_after: None,
            ending_before: None,
            limit: None,
            version: String::from("2022-04-06~experimental")
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Response {
    pub jsonapi: JsonAPI,
    pub data: Vec<Issue>,
    pub links: Links
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct JsonAPI {
    pub version: String
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Issue {
    pub r#type: String,
    pub id: String,
    pub attributes: Attributes,
    pub links: Links
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub issue_type: String,
    pub title: String,
    pub severity: Severity,
    pub ignored: bool,
    pub cwe: Vec<String>
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub own: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub first: Option<String>,
    pub last: Option<String>,
    pub related: Option<String>
}