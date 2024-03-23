# GitLab S3 Releaser

[![Build](https://github.com/theirix/gitlab-s3-releaser/actions/workflows/build.yml/badge.svg)](https://github.com/theirix/gitlab-s3-releaser/actions/workflows/build.yml)

A tool to create GitLab releases from versioned files stored in S3 bucket

S3 object template example:

tarballs/v([0-9\.]+)_.*/release.tar.gz
