use chrono::{self, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedIssuesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_description: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    include_introduced_through: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Filters>,
}

impl AggregatedIssuesRequest {
    pub fn new() -> Self {
        Self {
            include_description: None,
            include_introduced_through: None,
            filters: None,
        }
    }

    pub fn include_description(mut self, enable: bool) -> Self {
        self.include_description = Some(enable);
        self
    }

    pub fn include_introduced_through(mut self, enable: bool) -> Self {
        self.include_introduced_through = Some(enable);
        self
    }

    pub fn filters(mut self, filters: Filters) -> Self {
        self.filters = Some(filters);
        self
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    #[serde(skip_serializing_if = "Option::is_none")]
    severities: Option<Vec<Severity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exploit_maturity: Option<ExploitMaturity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<Vec<Type>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ignored: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    patched: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PriorityScore>,
}

impl Filters {
    pub fn new() -> Self {
        Self {
            severities: None,
            exploit_maturity: None,
            types: None,
            ignored: None,
            patched: None,
            priority: None
        }
    }

    pub fn severities(mut self, severities: Vec<Severity>) -> Self {
        self.severities = Some(severities);
        self
    }

    pub fn exploit_maturity(mut self, exploit_maturity: ExploitMaturity) -> Self {
        self.exploit_maturity = Some(exploit_maturity);
        self
    }

    pub fn types(mut self, types: Vec<Type>) -> Self {
        self.types = Some(types);
        self
    }

    pub fn ignored(mut self, ignored: bool) -> Self {
        self.ignored = Some(ignored);
        self
    }

    pub fn patched(mut self, patched: bool) -> Self {
        self.patched = Some(patched);
        self
    }

    pub fn priority(mut self, priority: PriorityScore) -> Self {
        self.priority = Some(priority);
        self
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExploitMaturity {
    Mature,
    ProofOfConcept,
    NoKnownExploit,
    NoData,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Vuln,
    License,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityScore {
    score: Score,
}

impl PriorityScore {
    pub fn new(min: usize, max: usize) -> Self {
        let score = Score::new(min, max);

        Self {
            score
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    min: usize,
    max: usize,
}

impl Score {
    pub fn new(min: usize, max: usize) -> Self {
        Self {
            min,
            max
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issues {
    pub issues: Vec<Issue>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub issue_type: String,
    pub pkg_name: String,
    pub pkg_versions: Vec<String>,
    pub issue_data: IssueData,
    pub introduced_through: Option<Vec<IntroducedThrough>>,
    pub is_patched: bool,
    pub is_ignored: bool,
    pub ignore_reasons: Option<Vec<IgnoreReason>>,
    pub fix_info: FixInfo,
    pub priority: Option<Priority>,
    pub links: Option<Links>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueData {
    pub id: String,
    pub title: String,
    pub severity: String,
    pub original_severity: Option<String>,
    pub url: String,
    pub description: Option<String>,
    pub identifiers: Option<Identifiers>,
    pub credit: Option<Vec<String>>,
    pub exploit_maturity: Option<String>,
    pub semver: Semver,
    pub publication_time: Option<chrono::DateTime<Utc>>,
    pub disclosure_time: Option<chrono::DateTime<Utc>>,

    #[serde(rename = "CVSSv3")]
    pub cvssv3: Option<String>,

    pub cvss_score: Option<f64>,
    pub language: String,
    pub patches: Option<Vec<Patch>>,
    pub nearest_fixed_in_version: String,
    pub path: Option<String>,
    pub violated_policy_public_id: Option<String>,
    pub is_malicious_package: Option<bool>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifiers {
    #[serde(rename = "CVE")]
    pub cve: Option<Vec<String>>,

    #[serde(rename = "CWE")]
    pub cwe: Option<Vec<String>>,

    #[serde(rename = "OSVDB")]
    pub osvdb: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Semver {
    // can be empty string
    pub vulnerable: Vulnerable,
    pub unaffected: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Vulnerable {
    Version(String),
    Versions(Vec<String>),
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patch {
    pub id: String,
    pub urls: Vec<String>,
    pub version: String,
    pub comments: Vec<String>,
    pub modification_time: chrono::DateTime<Utc>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntroducedThrough {
    pub kind: String,
    pub data: Data,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IgnoreReason {
    pub reason: String,
    pub expires: Option<String>,
    pub source: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixInfo {
    pub is_upgradable: bool,
    pub is_pinnable: bool,
    pub is_patchable: bool,
    pub is_fixable: bool,
    pub is_partially_fixable: bool,
    pub nearest_fixed_in_version: String,
    pub fixed_in: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    pub score: i64,
    pub factors: Vec<Factor>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Factor {
    name: String,
    description: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub paths: String,
}
