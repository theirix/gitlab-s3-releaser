use crate::s3indexer::S3Indexer;
use log::info;

pub async fn main_runner(
    bucket: String,
    path_template: String,
    s3_endpoint_url: Option<String>,
) -> anyhow::Result<()> {
    let indexer = S3Indexer::new(bucket, path_template, s3_endpoint_url).await?;
    let artifacts = indexer.list().await?;
    for artifact in artifacts {
        info!("Found artifact {artifact:?} in S3");
    }
    Ok(())
}
