use anyhow;
use snyk_data;
use structopt::StructOpt;
use futures::{stream, StreamExt};
use crate::entities::{self, container_issues::FromModel};
use std::{sync::{Arc, Mutex}, str::FromStr};

#[derive(Debug, PartialEq, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

impl Opt {
    pub async fn run(self, datasource: &dyn snyk_data::Datasource) -> anyhow::Result<()> {
        match self.cmd {
            Command::Test(opt) => {
                let issues = datasource.list_aggregated_sca_container_iac_issues(&opt.org_id, &opt.project_id).await?;
                issues.issues.iter()
                    .for_each(|issue| println!("{}", issue.issue_type));

                Ok(())
            },
            Command::ListByOrg(opt) => {
                let project_properties = snyk_data::model::projects::ListProjectsRequest::new();
                let projects = datasource.list_projects(&opt.org_id, &project_properties).await?;
                
                let projects_with_issues = projects.projects.iter()
                    .filter(|project| {
                        if let Some(min_severity) = &opt.min_severity {
                            match min_severity {
                                Severity::Critical => project.issue_counts_by_severity.critical > 0,
                                Severity::High => project.issue_counts_by_severity.critical > 0 ||
                                                  project.issue_counts_by_severity.high > 0,
                                Severity::Medium => project.issue_counts_by_severity.critical > 0 ||
                                                    project.issue_counts_by_severity.high > 0 ||
                                                    project.issue_counts_by_severity.medium > 0,
                                Severity::Low => project.issue_counts_by_severity.critical > 0 ||
                                                 project.issue_counts_by_severity.high > 0 ||
                                                 project.issue_counts_by_severity.medium > 0 ||
                                                 project.issue_counts_by_severity.low > 0
                            }
                        } else {
                            project.issue_counts_by_severity.critical > 0 ||
                            project.issue_counts_by_severity.high > 0 ||
                            project.issue_counts_by_severity.medium > 0 ||
                            project.issue_counts_by_severity.low > 0
                        }
                    })
                    .collect::<Vec<&snyk_data::model::projects::Project>>();

                let issues = Arc::new(Mutex::new(entities::issues::Issues::new()));

                stream::iter(projects_with_issues.iter())
                    .filter(|project| async {
                        project.r#type != "sast"
                    })
                    .map(|project| async {
                        let project_issues = datasource.list_aggregated_sca_container_iac_issues(&opt.org_id, &project.id).await.unwrap();
                        let issues = issues.clone();

                        // Issues from a docker container
                        if project.r#type == "dockerfile" {
                            let mut new_issues = entities::container_issues::ContainerIssues::from_model(project_issues);
                            let mut issues = issues.lock().unwrap();

                            if let Some(container_issues) = &mut issues.container_issues {
                                container_issues.append(&mut new_issues);
                            } else {
                                issues.container_issues = Some(new_issues);
                            }
                        } else {
                            // Issues from either sca or iac
                            project_issues.issues.into_iter()
                                .for_each(|issue| {
                                    if issue.issue_type == "configuration" {
                                        // Issues from IaC
                                        let new_issue = entities::iac_issues::IACIssue::from_model(issue);
                                        let mut issues = issues.lock().unwrap();

                                        if let Some(iac_issues) = &mut issues.iac_issues {
                                            iac_issues.push(new_issue);
                                        } else {
                                            issues.iac_issues = Some(vec![new_issue]);
                                        }
                                    } else {
                                        // Issues from SCA
                                        // Either license or vulnerability
                                        let new_issue = entities::sca_issues::SCAIssue::from_model(issue);
                                        let mut issues = issues.lock().unwrap();

                                        if let Some(sca_issues) = &mut issues.sca_issues {
                                            sca_issues.push(new_issue);
                                        } else {
                                            issues.sca_issues = Some(vec!(new_issue));
                                        }
                                    }
                                });
                        }
                    })
                    .buffer_unordered(10)
                    .collect::<()>()
                    .await;

                let sast_properties = snyk_data::model::issue_v3::SnykCodeIssuesRequest::new();
                stream::iter(projects_with_issues.iter())
                    .filter(|project| async {
                        project.r#type == "sast"
                    })
                    .map(|project| async {
                        let sast_issues = datasource.list_sast_issues(&opt.org_id, &project.id, &sast_properties).await.unwrap();
                        stream::iter(sast_issues
                            .data
                            .iter())
                            .map(|sast_issue| async {
                                let sast_issue_details = datasource.list_sast_issue_details(&sast_issue.links.own.as_ref().unwrap()).await.unwrap();
                                let sast_issue = entities::sast_issue::SastIssue::from_model(sast_issue, sast_issue_details);
                                let issues = issues.clone();
                                let mut issues = issues.lock().unwrap();
                                
                                if let Some(sast_issues) =  &mut issues.sast_issues {
                                    sast_issues.push(sast_issue);
                                } else {
                                    issues.sast_issues = Some(vec![sast_issue]);
                                }
                            })
                            .buffer_unordered(10)
                            .collect::<()>()
                            .await;
                    })
                    .buffer_unordered(10)
                    .collect::<()>()
                    .await;

                println!("{}", serde_json::to_string(&issues.as_ref()).unwrap());

                Ok(())
            }
            Command::ListCode(opt) => {
                let properties = snyk_data::model::issue_v3::SnykCodeIssuesRequest::new();
                let mut response = datasource
                    .list_sast_issues(&opt.org_id, &opt.project_id, &properties)
                    .await?;
                let mut issues = response.data;

                // if response.links.next exists then paginate issues
                if let Some(next_link) = response.links.next {
                    loop {
                        response = datasource.next(&next_link).await?;
                        issues.append(&mut response.data);

                        if response.links.next.is_none() {
                            break;
                        };
                    }
                };

                dbg!(&issues);

                let detail_url = &issues.first().unwrap().links.own.as_ref().unwrap();

                let detail_response = datasource.list_sast_issue_details(&detail_url).await?;

                dbg!(detail_response);

                Ok(())
            }
        }
    }
}

#[derive(Debug, PartialEq, StructOpt)]
enum Command {
    Test(Test),
    ListByOrg(ListByOrg),
    ListCode(ListCode),
}

#[derive(Debug, PartialEq, StructOpt)]
struct Test {
    org_id: String,
    project_id: String
}

#[derive(Debug, PartialEq, StructOpt)]
struct ListByOrg {
    #[structopt(long, short)]
    org_id: String,

    #[structopt(long, short)]
    min_severity: Option<Severity>,
}

#[derive(Debug, PartialEq, StructOpt)]
struct ListCode {
    #[structopt(long, short)]
    org_id: String,

    #[structopt(long, short)]
    project_id: String,
}

#[derive(Debug, PartialEq, StructOpt)]
enum Severity {
    Critical,
    High,
    Medium,
    Low
}

impl FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "critical" => Ok(Self::Critical),
            "high" => Ok(Self::High),
            "medium" => Ok(Self::Medium),
            "low" => Ok(Self::Low),
            _ => Err(String::from("Unsupported severity"))
        }
    }
}