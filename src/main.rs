use clap::Parser;
use gitlab_s3_releaser::releaser::main_runner;
use log::{error, info};

#[derive(Debug, Parser)]
struct Opt {
    /// Bucket name
    #[arg(short, long)]
    bucket: String,

    /// S3 path template regex
    #[arg(short, long)]
    path_template: String,

    // S3 endpoint URL
    #[arg(short, long)]
    s3_endpoint_url: Option<String>,

    /// Whether to run without sending to CloudWatch
    #[arg(short, long)]
    dryrun: bool,
}

#[tokio::main]
#[allow(clippy::result_large_err)]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env().init();

    let opt = Opt::parse();
    let result = main_runner(opt.bucket, opt.path_template, opt.s3_endpoint_url).await;
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
