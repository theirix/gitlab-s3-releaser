use crate::gitlabreleases::GitlabReleases;
use crate::s3indexer::S3Indexer;
use log::info;

pub async fn main_runner(
    bucket: String,
    path_template: String,
    s3_endpoint_url: Option<String>,
    gitlab_host: String,
    gitlab_token: String,
    project: String,
) -> anyhow::Result<()> {
    let gitlab_releases = GitlabReleases::new(gitlab_host, gitlab_token, project).await?;
    let releases = gitlab_releases.list_releases().await?;
    info!("Releases: {releases:?}");

    let indexer = S3Indexer::new(bucket, path_template, s3_endpoint_url).await?;
    let artifacts = indexer.list().await?;
    for artifact in artifacts {
        info!("Found artifact {artifact:?} in S3");
    }

    Ok(())
}
