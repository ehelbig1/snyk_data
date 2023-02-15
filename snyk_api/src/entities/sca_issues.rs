use serde::Serialize;
use snyk_data;
use chrono::{self, Utc};

#[derive(Debug, PartialEq, Serialize)]
pub struct SCAIssue {
    issue_type: String,
    pkg_name: String,
    pkg_versions: Vec<String>,
    title: String,
    severity: String,
    description: Option<String>,
    cve: Option<Vec<String>>,
    cwe: Option<Vec<String>>,
    osvdb: Option<Vec<String>>,
    exploit_maturity: Option<String>,
    vulnerable_versions: Vulnerable,
    unaffected_versions: Option<String>,
    publication_time: Option<chrono::DateTime<Utc>>,
    language: String,
    //patches: Option<Vec<Patch>>,
    nearest_fixed_version: String,
    fixed_versions: Option<Vec<String>>,
    is_malicious: Option<bool>,
    is_patched: bool,
    is_ignored: bool,
    is_upgradeable: bool,
    is_pinnable: bool,
    is_patchable: bool,
    is_fixable: bool,
    is_partially_fixable: bool,
}

impl SCAIssue {
    pub fn from_model(model: snyk_data::model::issue::Issue) -> Self {
        Self {
            issue_type: model.issue_type,
            pkg_name: model.pkg_name,
            pkg_versions: model.pkg_versions,
            title: model.issue_data.title,
            severity: model.issue_data.severity,
            description: model.issue_data.description,
            cve: if let Some(identifiers) = &model.issue_data.identifiers {
                identifiers.cve.to_owned()
            } else {
                None
            },

            cwe: if let Some(identifiers) = &model.issue_data.identifiers {
                identifiers.cwe.to_owned()
            } else {
                None
            },

            osvdb: if let Some(identifiers) = &model.issue_data.identifiers {
                identifiers.osvdb.to_owned()
            } else {
                None
            },

            exploit_maturity: model.issue_data.exploit_maturity,
            vulnerable_versions: Vulnerable::from_model(model.issue_data.semver.vulnerable),
            unaffected_versions: model.issue_data.semver.unaffected,
            publication_time: model.issue_data.publication_time,
            language: model.issue_data.language,
            //patches: if let Some(patch) = model.issue_data.patches,
            nearest_fixed_version: model.issue_data.nearest_fixed_in_version,
            fixed_versions: model.fix_info.fixed_in,
            is_malicious: model.issue_data.is_malicious_package,
            is_patched: model.is_patched,
            is_ignored: model.is_ignored,
            is_upgradeable: model.fix_info.is_upgradable,
            is_pinnable: model.fix_info.is_pinnable,
            is_patchable: model.fix_info.is_patchable,
            is_fixable: model.fix_info.is_fixable,
            is_partially_fixable: model.fix_info.is_partially_fixable
        }
    }
}

// #[derive(Debug, PartialEq, Serialize)]
// pub enum Severity {
//     Critical,
//     High,
//     Medium,
//     Low,
// }

// impl Severity {
//     pub fn from_model(model: snyk_data::model::issue::Severity) -> Self {
//         match model {
//             snyk_data::model::issue::Severity::Critical => Self::Critical,
//             snyk_data::model::issue::Severity::High => Self::High,
//             snyk_data::model::issue::Severity::Medium => Self::Medium,
//             snyk_data::model::issue::Severity::Low => Self::Low
//         }
//     }
// }

#[derive(Debug, PartialEq, Serialize)]
pub enum Vulnerable {
    Version(String),
    Versions(Vec<String>),
}

impl Vulnerable {
    pub fn from_model(model: snyk_data::model::issue::Vulnerable) -> Self {
        match model {
            snyk_data::model::issue::Vulnerable::Version(version) => Self::Version(version),
            snyk_data::model::issue::Vulnerable::Versions(versions) => Self::Versions(versions)
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Patch {
    pub version: String,
    pub comments: Vec<String>,
    pub modification_time: chrono::DateTime<Utc>,
}