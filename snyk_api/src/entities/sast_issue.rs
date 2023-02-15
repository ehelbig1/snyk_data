use serde::Serialize;
use snyk_data;

impl SastIssue {
    pub fn from_model(issue_model: &snyk_data::model::issue_v3::Issue, detail_model: snyk_data::model::sast_issue_details::Response) -> Self {
        Self {
            title: issue_model.attributes.title.clone(),
            severity: Severity::from_model(&issue_model.attributes.severity),
            ignored: issue_model.attributes.ignored,
            cwe: issue_model.attributes.cwe.clone(),
            detail: detail_model.data.attributes.title,
            primary_file_path: detail_model.data.attributes.primary_file_path,
            start_line: None,
            end_line: None,
            severity_factors: detail_model.data.attributes.priority_score_factors
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SastIssue {
    title: String,
    severity: Severity,
    ignored: bool,
    cwe: Vec<String>,
    detail: String,
    primary_file_path: Option<String>,
    start_line: Option<usize>,
    end_line: Option<usize>,
    severity_factors: Option<Vec<String>>
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low
}

impl Severity {
    pub fn from_model(model: &snyk_data::model::issue_v3::Severity) -> Self {
        match model {
            snyk_data::model::issue_v3::Severity::Critical => Self::Critical,
            snyk_data::model::issue_v3::Severity::High => Self::High,
            snyk_data::model::issue_v3::Severity::Medium => Self::Medium,
            snyk_data::model::issue_v3::Severity::Low => Self::Low
        }
    }
}