use anyhow;
use snyk_data;
use structopt::StructOpt;
use serde_json;
use crate::entities::orgs::FromModel;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

impl Opt {
    pub async fn run(self, datasource: &dyn snyk_data::Datasource) -> anyhow::Result<()> {
        match self.cmd {
            Command::List(opt) => {
                let mut orgs = crate::entities::orgs::Orgs::from_model(datasource.list_orgs().await?);

                orgs = if let Some(name) = opt.name {
                    orgs
                        .into_iter()
                        .filter(|org| org.name.to_lowercase().contains(&name))
                        .collect()
                } else {
                    orgs
                };

                println!("{}", serde_json::to_string(&orgs).unwrap());

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
    /// Lists orgs that contain <name>
    #[structopt(short, long)]
    name: Option<String>,
}
