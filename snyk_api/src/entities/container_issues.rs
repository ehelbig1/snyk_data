use serde::Serialize;
use snyk_data;

pub trait FromModel {
    fn from_model(model: snyk_data::model::issue::Issues) -> Self;
}

pub type ContainerIssues = Vec<ContainerIssue>;

impl FromModel for ContainerIssues {
    fn from_model(model: snyk_data::model::issue::Issues) -> Self {
        model.issues.into_iter()
            .map(|mut issue| {
                ContainerIssue {
                    pkg_name: issue.pkg_name,
                    pkg_versions: issue.pkg_versions,
                    title: issue.issue_data.title,
                    severity: issue.issue_data.severity,
                    description: issue.issue_data.description,
                    cve: if let Some(identifiers) = &mut issue.issue_data.identifiers {
                        identifiers.cve.to_owned()
                    } else {
                        None
                    },

                    cwe: if let Some(identifiers) = &mut issue.issue_data.identifiers {
                        identifiers.cwe.to_owned()
                    } else {
                        None
                    },

                    osvdb: if let Some(identifiers) = &mut issue.issue_data.identifiers {
                        identifiers.osvdb.to_owned()
                    } else {
                        None
                    },

                    exploit_maturity: issue.issue_data.exploit_maturity,
                    language: issue.issue_data.language,
                    nearest_fixed_in_version: issue.issue_data.nearest_fixed_in_version,
                    is_patched: issue.is_patched,
                    is_ignored: issue.is_ignored,
                    is_upgradeable: issue.fix_info.is_upgradable,
                    is_pinnable: issue.fix_info.is_pinnable,
                    is_patchable: issue.fix_info.is_patchable,
                    is_fixable: issue.fix_info.is_fixable,
                    is_partially_fixable: issue.fix_info.is_partially_fixable
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ContainerIssue {
    pkg_name: String,
    pkg_versions: Vec<String>,
    title: String,
    severity: String,
    description: Option<String>,
    cve: Option<Vec<String>>,
    cwe: Option<Vec<String>>,
    osvdb: Option<Vec<String>>,
    exploit_maturity: Option<String>,
    //vulnerable_versions: Vec<String>,
    language: String,
    nearest_fixed_in_version: String,
    is_patched: bool,
    is_ignored: bool,
    is_upgradeable: bool,
    is_pinnable: bool,
    is_patchable: bool,
    is_fixable: bool,
    is_partially_fixable: bool,
    //severity_factors: Option<Vec<SeverityFactor>>
}

// #[derive(Debug, PartialEq, Serialize)]
// pub enum Severity {
//     Critical,
//     High,
//     Medium,
//     Low
// }

// impl Severity {
//     fn from_model(model: snyk_data::model::issue::Severity) -> Self {
//         match model {
//             Critical => Self::Critical,
//             High => Self::High,
//             Medium => Self::Medium,
//             Low => Self::Low
//         }
//     }
// }

// pub type SeverityFactors = Vec<SeverityFactor>;


// #[derive(Debug, PartialEq, Serialize)]
// pub struct SeverityFactor {
//     name: String,
//     description: String
// }

// impl SeverityFactor {
//     fn from_model(model: snyk_data::model::issue::Priority) -> Self {
//         model.factors.into_iter()
//             .map
//     }
// }