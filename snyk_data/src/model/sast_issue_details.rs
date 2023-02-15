use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub struct Properties {
    pub version: String,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            version: String::from("2022-04-06~experimental"),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Response {
    pub data: Data,
    pub jsonapi: JsonAPI,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Data {
    pub attributes: Attributes,
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub cwe: Vec<String>,
    pub ignored: bool,
    pub issue_type: String,
    pub severity: Severity,
    pub title: String,
    pub fingerprint: Option<String>,
    pub fingerprint_version: Option<String>,
    pub primary_file_path: Option<String>,
    pub primary_region: Option<PrimaryRegion>,
    pub priority_score: Option<usize>,
    pub priority_score_factors: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryRegion {
    pub end_line: usize,
    pub end_column: usize,
    pub start_line: usize,
    pub start_column: usize,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct JsonAPI {
    pub version: String,
}
