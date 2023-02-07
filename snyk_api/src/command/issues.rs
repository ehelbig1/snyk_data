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
                let issues = datasource
                    .list_aggregated_issues(&opt.org_id, &opt.project_id)
                    .await?;
                dbg!(issues);

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
    #[structopt(long, short)]
    org_id: String,

    #[structopt(long, short)]
    project_id: String,
}
