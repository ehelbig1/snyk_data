use serde::Serialize;
use snyk_data;
use chrono::{self, Utc};

#[derive(Debug, PartialEq, Serialize)]
pub struct IACIssue {
    title: String,
    severity: String,
    description: Option<String>,
    disclosure_time: Option<chrono::DateTime<Utc>>,
    path: Option<String>
}

impl IACIssue {
    pub fn from_model(model: snyk_data::model::issue::Issue) -> Self {
        Self {
            title: model.issue_data.title,
            severity: model.issue_data.severity,
            description: model.issue_data.description,
            disclosure_time: model.issue_data.disclosure_time,
            path: model.issue_data.path
        }
    }
}