use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Issues {
    pub sca_issues: Option<Vec<super::sca_issues::SCAIssue>>,
    pub container_issues: Option<super::container_issues::ContainerIssues>,
    pub iac_issues: Option<Vec<super::iac_issues::IACIssue>>,
    pub sast_issues: Option<Vec<super::sast_issue::SastIssue>>,
}

impl Issues {
    pub fn new() -> Self {
        Self {
            sca_issues: None,
            container_issues: None,
            iac_issues: None,
            sast_issues: None
        }
    }
}