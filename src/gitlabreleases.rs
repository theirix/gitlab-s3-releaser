use crate::artifact::Artifact;
use anyhow::Context;
use gitlab::api::{self, projects, AsyncQuery};
use gitlab::{AsyncGitlab, GitlabBuilder};
use log::{debug, info};
use serde::Deserialize;
use std::borrow::Cow;
use std::path::Path;

pub struct GitlabReleases {
    client: AsyncGitlab,
    project_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    id: i64,
    name: String,
    path_with_namespace: String,
}

impl GitlabReleases {
    pub async fn new(
        gitlab_host: String,
        token: String,
        project_name: String,
    ) -> anyhow::Result<Self> {
        let client = GitlabBuilder::new(gitlab_host, token)
            .build_async()
            .await
            .context("Cannot create Gitlab client")?;

        Ok(Self {
            client,
            project_name,
        })
    }

    pub async fn list_releases(&self) -> anyhow::Result<Vec<Release>> {
        let project_endpoint = projects::Project::builder()
            .project(&self.project_name)
            .build()
            .context("Build list projects endpoint")?;
        let project: Project = project_endpoint.query_async(&self.client).await?;
        info!("Project found, id {}, name {}", project.id, project.name);

        let releases_endpoint = projects::releases::ProjectReleases::builder()
            .project(project.path_with_namespace)
            .build()
            .context("Build list releases endpoint")?;
        let releases: Vec<Release> = api::paged(releases_endpoint, api::Pagination::All)
            .query_async(&self.client)
            .await?;
        debug!("Releases {releases:?}");
        Ok(releases)
    }

    pub async fn add_release(
        &self,
        project: Project,
        release: Release,
        artifact: Artifact,
    ) -> anyhow::Result<()> {
        let create_release_endpoint = projects::releases::CreateRelease::builder()
            .project(project.path_with_namespace)
            .name(format!("Release {}", artifact.version))
            .tag_name(release.tag_name)
            .tag_message("Autogenerated release from S3")
            .build()
            .context("Build create release endpoint")?;
        let _ = create_release_endpoint.query_async(&self.client).await?;
        Ok(())
    }

    pub async fn add_package(
        &self,
        project: Project,
        package_name: String,
        artifact: Artifact,
    ) -> anyhow::Result<()> {
        let contents = &b"TODO"[..];
        let file_name: Cow<str> = Cow::from(
            Path::new(&artifact.s3_path)
                .file_name()
                .context("Cannot parse S3 path")?
                .to_str()
                .context("Invalid S3 path")?,
        );
        let create_release_endpoint = projects::packages::generic::UploadPackageFile::builder()
            .project(project.path_with_namespace)
            .package_name(package_name)
            .package_version(artifact.version)
            .file_name(file_name)
            .contents(contents)
            .build()
            .context("Build upload package endpoint")?;
        let _ = create_release_endpoint.query_async(&self.client).await?;
        Ok(())
    }
}
