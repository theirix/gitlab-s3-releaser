use anyhow::bail;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use log::warn;

pub struct S3Indexer {
    client: Client,
    bucket: String,
    path_template: String,
}

#[derive(Debug)]
pub struct Artifact {
    s3_path: String,
    version: String,
}

impl S3Indexer {
    pub async fn new(
        bucket: String,
        path_template: String,
        s3_endpoint_url: Option<String>,
    ) -> anyhow::Result<Self> {
        let client = Self::create_client(s3_endpoint_url).await;
        Ok(Self {
            bucket,
            path_template,
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
        let client = Client::new(&config);
        client
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Artifact>> {
        let objects = self.list_objects().await?;
        let artifacts: Vec<Artifact> = objects
            .iter()
            .filter_map(|s3_path| match self.matcher(&s3_path) {
                Some(version) => Some(Artifact {
                    s3_path: s3_path.clone(),
                    version,
                }),
                None => None,
            })
            .collect();
        Ok(artifacts)
    }
    fn matcher(&self, _path: &String) -> Option<String> {
        Some("123".into())
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
