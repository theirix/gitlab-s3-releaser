use anyhow::Context;
use std::path::Path;

pub type Version = String;

#[derive(Debug)]
pub struct Artifact {
    pub s3_path: String,
    pub version: Version,
}

impl Artifact {
    pub fn file_name(&self) -> anyhow::Result<String> {
        Ok(Path::new(&self.s3_path.to_string())
            .file_name()
            .context("Cannot parse S3 path")?
            .to_str()
            .context("Invalid S3 path")?
            .to_string())
    }
}
