use serde::Serialize;
use snyk_data;
use chrono::{self, Utc};

pub trait FromModel {
    fn from_model(model: snyk_data::model::projects::Projects) -> Self;
}

pub type Projects = Vec<Project>;

impl FromModel for Projects {
    fn from_model(model: snyk_data::model::projects::Projects) -> Self {
        model.projects.into_iter()
            .map(|project| {
                Project {
                    name: project.name,
                    id: project.id,
                    origin: project.origin,
                    r#type: project.r#type,
                    total_dependencies: project.total_dependencies,
                    issue_counts_by_severity: IssueCountsBySeverity::from_model(project.issue_counts_by_severity),
                    last_tested_date: project.last_tested_date,
                    is_monitored: project.is_monitored,
                    branch: project.branch
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Project {
    name: String,
    id: String,
    origin: String,
    r#type: String,
    total_dependencies: Option<usize>,
    issue_counts_by_severity: IssueCountsBySeverity,
    last_tested_date: chrono::DateTime<Utc>,
    is_monitored: bool,
    branch: Option<String>
}

#[derive(Debug, PartialEq, Serialize)]
pub struct IssueCountsBySeverity {
    critical: usize,
    high: usize,
    medium: usize,
    low: usize
}

impl IssueCountsBySeverity {
    fn from_model(model: snyk_data::model::projects::IssueCountsBySeverity) -> Self {
        Self {
            critical: model.critical,
            high: model.high,
            medium: model.medium,
            low: model.low
        }
    }
}