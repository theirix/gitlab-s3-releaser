use anyhow::Context;
use clap::Parser;
use gitlab_s3_releaser::releaser::main_runner;
use log::{error, info};
use std::env;

#[derive(Debug, Parser)]
struct Opt {
    /// Bucket name
    #[arg(short, long)]
    bucket: String,

    /// S3 path template regex
    #[arg(long)]
    path_template: String,

    // S3 endpoint URL
    #[arg(long)]
    s3_endpoint_url: Option<String>,

    // GitLab host
    #[arg(long)]
    gitlab_host: String,

    // GitLab project
    #[arg(long)]
    project: String,

    /// Whether to run without sending to CloudWatch
    #[arg(short, long)]
    dryrun: bool,
}

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env().init();

    let gitlab_token =
        env::var("GITLAB_TOKEN").context("Specify Gitlab token in GITLAB_TOKEN env var")?;
    let opt = Opt::parse();
    let result = main_runner(
        opt.bucket,
        opt.path_template,
        opt.s3_endpoint_url,
        opt.gitlab_host,
        gitlab_token,
        opt.project,
    )
    .await;
    match result {
        Ok(_) => {
            info!("Done");
            Ok(())
        }
        Err(err) => {
            error!("Error: {}", err);
            Err(err)
        }
    }
}
