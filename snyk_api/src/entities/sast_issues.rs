use serde::Serialize;
use snyk_data;

pub trait FromModel {
    fn from_model(issue_model: snyk_data::model::issue_v3::Response, detail_model: snyk_data::model::sast_issue_details::Response) -> Vec<SastIssue>;
}

pub type SastIssues = Vec<SastIssue>;

// impl FromModel for SastIssues {
//     fn from_model(issue_model: snyk_data::model::issue_v3::Response, detail_model: snyk_data::model::sast_issue_details::Response) -> Vec<SastIssue> {
        
//     }
// }

#[derive(Debug, PartialEq, Serialize)]
pub struct SastIssue {
    title: String,
    severity: Severity,
    ignored: bool,
    cwe: Vec<String>,
    detail: String,
    primary_file_path: String,
    start_line: usize,
    end_line: usize,
    severity_factors: Vec<String>
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low
}

impl Severity {
    pub fn from_model(model: snyk_data::model::issue_v3::Severity) -> Self {
        match model {
            Critical => Self::Critical,
            High => Self::High,
            Medium => Self::Medium,
            Low => Self::Low
        }
    }
}