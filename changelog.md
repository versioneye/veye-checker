# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - 2017-04-xx
### Added

- add `shas` command to calculate checksums of the binaries
- add `lookup` command to lookup a product details by a file digest
- add `resolve` command to shazam package binary into details
- add configs manager to read configuration from ENV vars or from the `veye_checker.toml`
- add csv output writers
- add release script for MS
- add release scripts for Linux
- add release scripts for OSx


### Fixed

- issue #12, error message was missing from output
- issue #7, execution raised panic when API response didnt match schema
- issue #1, show API errors in final output without stopping processing


