# Veye-Checker

Simple command-line util to scan Nupkg and Jar files and resolve their SHA values to package information.


## Usage

* **scan** - scans the target folder recursively and digest supported packagefiles into SHA

```bash

veye_checker scan ../temp/ 
veye_checker scan ../temp/ -o results.csv
```

* **lookup** - fetches product details from VersionEye api for generated SHA hash

```bash

veye_checker lookup <SHA_STRING> -a <YOUR_API_KEY>
```

* **lookup_csv** - looks up SHA values from the `scan` output file and 
tries to fetch product info from the Versioneye API. 

```bash
veye_checker lookup_csv scan_results.csv -a <YOUR_API_KEY> -o res.csv
```

## API keys

TODO

## Permanent configs

TODO

## Build

```bash
> cargo build
> ./target/debug/veye_checker

or simpler command
> cargo run

or optimized production release
> cargo build --release
> ./target/release/veye-checker
```