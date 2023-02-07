use anyhow;
use reqwest;
use snyk_data;
use std::env;
use structopt::StructOpt;

mod command;

#[derive(Debug, PartialEq, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, PartialEq, StructOpt)]
enum Command {
    Orgs(command::orgs::Opt),
    Issues(command::issues::Opt),
    Projects(command::projects::Opt),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = env::var("SNYK_API_KEY").expect("SNYK_API_KEY must be provided");
    let http_client = reqwest::Client::new();
    let datasource = snyk_data::SnykDatasource::new(&http_client, &api_key);

    let opt = Opt::from_args();
    match opt.cmd {
        Command::Orgs(opt) => {
            opt.run(&datasource).await?;
        }
        Command::Issues(opt) => {
            opt.run(&datasource).await?;
        }
        Command::Projects(opt) => {
            opt.run(&datasource).await?;
        }
    }

    Ok(())
}
