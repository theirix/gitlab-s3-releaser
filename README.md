# GitLab S3 Releaser

[![Build](https://github.com/theirix/gitlab-s3-releaser/actions/workflows/build.yml/badge.svg)](https://github.com/theirix/gitlab-s3-releaser/actions/workflows/build.yml)

A tool to create GitLab releases from versioned files stored in S3 bucket.

## Installation

    cargo install gitlab-s3-releaser

## Usage

Typical usage - publish some external CI-created artifacts to GitLab releases if an internal GitLab CI is not used.

The releaser scans objects in the S3 bucket with a regular expression and finds artifacts. The version of artifact is deduced from `version` in the regex. For all these artifacts, a binary package and a corresponding release are created for the specific project (parameter `project`).

Deduce S3 object template example with `version` named group, for example:

    tarballs\/v(?<version>[0-9\.]+)[^\/]*\/.*

Invoke releaser

    gitlab-s3-releaser --bucket s3-bucket.tld.org --package=release \
      --path-template "tarballs\/v(?<version>[0-9\.]+)[^\/]*\/.*" \
      --gitlab-host=gitlab.tld.org --project="dev/gitlab-project" 

To view more logs, specify `RUST_LOG=info` or `debug` env variable.
