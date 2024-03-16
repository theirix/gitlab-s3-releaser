use crate::artifact::{Artifact, Version};
use anyhow::{bail, Context};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use log::warn;
use regex::Regex;

pub struct S3Indexer {
    client: Client,
    bucket: String,
    path_template_regex: Regex,
}

impl S3Indexer {
    pub async fn new(
        bucket: String,
        path_template: String,
        s3_endpoint_url: Option<String>,
    ) -> anyhow::Result<Self> {
        // Create S3 client
        let client = Self::create_client(s3_endpoint_url).await;
        let path_template_regex = Regex::new(&path_template).context("Cannot parse regex")?;
        Ok(Self {
            bucket,
            path_template_regex,
            client,
        })
    }

    async fn create_client(s3_endpoint_url: Option<String>) -> Client {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let mut config_loader =
            aws_config::defaults(aws_config::BehaviorVersion::latest()).region(region_provider);

        if let Some(url) = s3_endpoint_url {
            config_loader = config_loader.endpoint_url(url);
        }
        let config = config_loader.load().await;
        Client::new(&config)
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Artifact>> {
        let objects = self.list_objects().await?;
        let artifacts: Vec<Artifact> = objects
            .iter()
            .filter_map(|s3_path| {
                self.matcher(s3_path).map(|version| Artifact {
                    s3_path: s3_path.clone(),
                    version,
                })
            })
            .collect();
        Ok(artifacts)
    }
    fn matcher(&self, path: &str) -> Option<Version> {
        if let Some(captures) = self.path_template_regex.captures(path) {
            match captures.name("version") {
                Some(version_capture) => return Some(Version::from(version_capture.as_str())),
                None => warn!(
                    "There is no 'version' named capture in regex {}",
                    self.path_template_regex
                ),
            }
        }
        // Not an error, just not matched path
        None
    }

    async fn list_objects(&self) -> anyhow::Result<Vec<String>> {
        let mut response = self
            .client
            .list_objects_v2()
            .bucket(self.bucket.to_owned())
            .max_keys(10)
            .into_paginator()
            .send();

        let mut paths: Vec<String> = vec![];

        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    for object in output.contents() {
                        match object.key() {
                            Some(path) => paths.push(path.into()),
                            None => warn!("Error accessing key"),
                        }
                    }
                }
                Err(err) => {
                    bail!("Error getting list of objects {err:?}")
                }
            }
        }

        Ok(paths)
    }
}
