# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased] v0.2.0 - 2017-06-2x
### Added 

 - issue #18 - possible to block digest algorithms and specify file extensions;
 - add new cmd-line options: `no-md5, no-sha1, no-sha512, ext-md5, ext-sha1, ext-sha512`
 
### Fixed
 
 - issue #13 - handle cases when 2 or more shas are returned from API;
 - issue #20 - config file requires all the top-level categories specified;
 - issue #17 - product URL was using host of SAAS, not from configs;

## v0.1.0 - 2017-04-24
### Added

- add `-c` flag to override default location of configuration file
- add `shas` command to calculate checksums of the binaries
- add `lookup` command to lookup a product details by a file digest
- add `resolve` command to shazam package binary into details
- add configs manager to read configuration from ENV vars or from the `veye_checker.toml`
- add csv output writers
- add release script for MS
- add release scripts for Linux
- add release scripts for OSx
- make output CSV configurable,  #11
- add support for Python PYPI files, #6
- add support for NPM files, #5
- add support for proxy, #14

### Fixed

- issue #12, error message was missing from output
- issue #7, execution raised panic when API response didnt match schema
- issue #1, show API errors in final output without stopping processing
- temporary fix for configs_test, which sometimes fail due the fact the manipulating ENV vars may have read/delete conflicts.
- fix nuget lookup when its base64 includes `+, /, =` which are not URL safe characters;


