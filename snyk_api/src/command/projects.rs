use anyhow;
use snyk_data;
use structopt::StructOpt;
use crate::entities::projects::FromModel;
use serde_json;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

impl Opt {
    pub async fn run(self, datasource: &dyn snyk_data::Datasource) -> anyhow::Result<()> {
        match self.cmd {
            Command::List(opt) => {
                let mut filters = snyk_data::model::projects::Filters::new();

                filters = if let Some(name) = opt.name {
                    filters.name(name)
                } else {
                    filters
                };

                filters = if let Some(origin) = opt.origin {
                    filters.origin(origin)
                } else {
                    filters
                };

                filters = if let Some(r#type) = opt.r#type {
                    filters.r#type(r#type)
                } else {
                    filters
                };

                let properties =
                    snyk_data::model::projects::ListProjectsRequest::new().filters(filters);

                let projects = crate::entities::projects::Projects::from_model(datasource.list_projects(&opt.org_id, &properties).await?);
                println!("{}", serde_json::to_string(&projects).unwrap());

                Ok(())
            }
        }
    }
}

#[derive(Debug, PartialEq, StructOpt)]
enum Command {
    List(List),
}

#[derive(Debug, PartialEq, StructOpt)]
struct List {
    org_id: String,

    /// Matches the beginning of the project name (case-sensitive)
    #[structopt(short, long)]
    name: Option<String>,

    /// Filter based on the origin of the scan (github, azure-repos, cli, etc.)
    #[structopt(short, long)]
    origin: Option<String>,

    /// Filter based on the type of project (npm, yarn, gradle, cocoapods, etc.)
    #[structopt(short, long)]
    r#type: Option<String>,

    /// Filter based on whether a project is monitored or not
    is_monitored: Option<bool>,
}
