# Veye-Checker


[![Join the chat at Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/veye_checker/Lobby?utm_source=share-link&utm_medium=link&utm_campaign=share-link)

It's a command-line util that scans packaged binaries (`*.nupkg, *.Jar`) and resolves their SHA digest values into package information.

One can use this utility to lookup package version details, license, vulnerability details or automate due diligence process without installing any runtime or additional dependencies.
 

## Usage

Download binaries from the [releases](https://github.com/versioneye/veye-checker/releases) and save it into your binaries folder

```
curl -s -L -o "${HOME}/bin/veye_checker"  https://github.com/versioneye/veye-checker/releases/download/v0.1.0-RC1/veye_checker_osx

chmod a+x ~/bin/veye_checker
```

* **resolve** - scans the target folder recursively, translates filedigest via VersionEye API into product details and prints out results.

```
veye_checker resolve ../jars -a "api-key"
VERSIONEYE_API_KEY="apitoken" veye_checker resolve ../jars
veye_checker resolve ../jars -o resolve.csv -a "api-key"
```

* **shas** - scans the target folder recursively and outputs digests of supported packagefiles:

```bash
veye_checker shas ../jars/ 
veye_checker shas ../jars/ -o results.csv
```

* **lookup** - fetches product details from VersionEye api by the SHA/digest value.

```bash
veye_checker lookup <SHA_STRING> -a <YOUR_API_KEY>
VERSIONEYE_API_KEY="apikey" veye_checker lookup <SHA_STRING>
```

## API keys

All the commands ( *lookup*, *resolve*, etc ) requesting data from the  [VersionEye api](https://www.versioneye.com/api/v2) require the API-key, which you can obtain from [your's profile page](https://www.versioneye.com/organisations/private/apikey).

It's possible to specify the api-key 3 ways:

* via environment variable `VERSIONEYE-API-KEY` 

```
export VERSIONEYE_API_KEY="abcdef1234" veye_checker lookup SHA_VALUE_123
```

* add `veye_checker.toml` config file:

```
[api]
key = "abcdef1234"
```

* specify explicitly via command parameter

```
veye_checker lookup SHA_VALUE_123 -a abcdef1234
```

## Configuration via ENV variable

It's possible to tweak a setting of the command-line tool with environmental variables, and all the variables follow a pattern: `VERSIONEYE_GROUPID_VARIABLEID`. 


| full id               | default value | description                |
|:---------------------:|---------------|----------------------------|
| VERSIONEYE\_API\_KEY  | None          | specifies API key for the Versioneye API|
| VERSIONEYE\_API\_HOST | www.versioneye.com | specifies custom host name for VersionEye API, useful when using hosted or enterprise version.|
| VERSIONEYE\_API\_PATH | api/v2        | specifies URL path between the host and REST resource |
| VERSIONEYE\_API\_PORT | None          | specifies port number for API |
| VERSIONEYE\_API\_SCHEME | https       | specifies URI scheme |

## Configuration via config file

One can put permanent configurations for the `veye_checker` tool into  `veye_checker.toml` file. All the fields in the configuration file are optional, and the commandline tool will use default values for unspecified fields.

```toml
# veye_checker.toml
[api]
key = "Your API key"
host = "www.versioneye.com"
path = "api/v2"
port = 80
scheme = "https"

```

## Build

```bash
> cargo build
> ./target/debug/veye_checker

or simpler command
> cargo run

or running tests
> cargo test
#test only api-calls
> VERSIONEYE_API_KEY="APIKEY" cargo test --features "api"

or optimized production release
> cargo build --release
> ./target/release/veye-checker

```

## Contributing

It's opensource project and any kind of contribution is more than welcome. 
