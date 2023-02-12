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
                    .list_aggregated_sca_container_iac_issues(&opt.org_id, &opt.project_id)
                    .await?;

                // how can we know from here if the issues are from a container, iac, or manifest file?
                // this information is one level higher, in the project itself
                dbg!(issues);

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
    List(List),
    ListCode(ListCode),
}

#[derive(Debug, PartialEq, StructOpt)]
struct List {
    #[structopt(long, short)]
    org_id: String,

    #[structopt(long, short)]
    project_id: String,
}

#[derive(Debug, PartialEq, StructOpt)]
struct ListCode {
    #[structopt(long, short)]
    org_id: String,

    #[structopt(long, short)]
    project_id: String,
}
