use anyhow;
use snyk_data;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

impl Opt {
    pub async fn run(self, datasource: &dyn snyk_data::Datasource) -> anyhow::Result<()> {
        match self.cmd {
            Command::List(opt) => {
                let orgs = datasource.list_orgs().await?;

                let orgs: Vec<snyk_data::model::org::Org> = if let Some(name) = opt.name {
                    orgs.orgs.into_iter()
                        .filter(|org| org.name.to_lowercase().contains(&name))
                        .collect()
                } else {
                    orgs.orgs
                };

                dbg!(orgs);

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
    #[structopt(short, long)]
    name: Option<String>
}
