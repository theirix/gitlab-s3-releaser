use anyhow::Context;
use gitlab::api::{self, projects, AsyncQuery};
use gitlab::{AsyncGitlab, GitlabBuilder};
use log::{debug, info};
use serde::Deserialize;
use crate::artifact::Artifact;

pub struct GitlabReleases {
    client: AsyncGitlab,
    project_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    name: String,
    tag_name: String
}

#[derive(Debug, Deserialize)]
pub struct Project {
    id: i64,
    name: String,
    path_with_namespace: String
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
        let releases: Vec<Release> = api::paged( releases_endpoint, api::Pagination::All).query_async(&self.client).await?;
        debug!("Releases {releases:?}");
        Ok(releases)
    }
}
