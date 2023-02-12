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
    data: Data,
    jsonapi: JsonAPI,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Data {
    attributes: Attributes,
    id: String,
    r#type: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    cwe: Vec<String>,
    ignored: bool,
    issue_type: String,
    severity: Severity,
    title: String,
    fingerprint: Option<String>,
    fingerprint_version: Option<String>,
    primary_file_path: Option<String>,
    primary_region: Option<PrimaryRegion>,
    priority_score: Option<usize>,
    priority_score_factors: Option<Vec<String>>,
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
    version: String,
}
