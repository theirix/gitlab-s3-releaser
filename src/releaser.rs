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
        releases.into_iter().map(|r| r.tag_name).collect();
    info!("Found Gitlab Releases: {existing_gitlab_releases:?}");

    let tags = gitlab_releases
        .list_tags()
        .await
        .context("Cannot list tags")?;
    info!("Found Gitlab tags: {tags:?}");

    for artifact in artifacts {
        let release_exists = existing_gitlab_releases.contains(&artifact.version);
        let tag_exists = tags.contains(&artifact.version);
        if release_exists {
            info!("Found already existing artifact {artifact:?} in GitLab");
        } else if !tag_exists {
            info!("Found artifact {artifact:?} in S3 but there is not tag in GitLab");
        } else {
            info!("Found new artifact {artifact:?} in S3");
            let contents = indexer
                .download(&artifact)
                .await
                .context("Cannot download S3 artifact")?;
            info!("Downloaded {} bytes", contents.len());

            gitlab_releases
                .add_package(&package_name, &artifact, contents)
                .await
                .context("Cannot add package")?;
            info!("Created package");

            let release = gitlab_releases
                .add_release(&package_name, &artifact)
                .await
                .context("Cannot add package")?;
            info!("Created release {}", release.name);
        }
    }
    Ok(())
}
