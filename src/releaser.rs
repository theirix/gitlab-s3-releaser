use crate::gitlabreleases::GitlabReleases;
use crate::s3indexer::S3Indexer;
use anyhow::Context;
use log::info;
use std::collections::HashSet;

pub async fn main_runner(
    bucket: String,
    path_template: String,
    s3_endpoint_url: Option<String>,
    gitlab_host: String,
    gitlab_token: String,
    project: String,
    package_name: String,
) -> anyhow::Result<()> {
    let gitlab_releases = GitlabReleases::new(gitlab_host, gitlab_token, project).await?;
    let releases = gitlab_releases
        .list_releases()
        .await
        .context("Cannot list releases")?;
    info!("Releases: {releases:?}");

    let indexer = S3Indexer::new(bucket, path_template, s3_endpoint_url).await?;
    let artifacts = indexer.list().await?;

    let existing_gitlab_releases: HashSet<String> =
        releases.iter().map(|r| r.tag_name.clone()).collect();
    info!("Found Gitlab Releases: {existing_gitlab_releases:?}");
    for artifact in artifacts {
        let exists = existing_gitlab_releases.contains(&artifact.version);
        if exists {
            info!("Found already existing artifact {artifact:?} in S3");
        } else {
            info!("Found new artifact {artifact:?} in S3");
            let contents = indexer
                .download(&artifact)
                .await
                .context("Cannot download S3 artifact")?;
            info!("Downloaded {} bytes", contents.len());

            gitlab_releases
                .add_package(&package_name, artifact, contents)
                .await
                .context("Cannot add package")?;
        }
    }
    Ok(())
}
