pub type Version = String;

#[derive(Debug)]
pub struct Artifact {
    pub s3_path: String,
    pub version: Version,
}
