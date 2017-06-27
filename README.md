# Veye-Checker


[![Join the chat at Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/veye_checker/Lobby?utm_source=share-link&utm_medium=link&utm_campaign=share-link)

It's a command-line util that scans packaged binaries and resolves their SHA digest values into the package information.
The whole idea behind this utility is described in the Versioneye's blogpost ["Identifying components by SHA values"](https://blog.versioneye.com/2017/02/08/identifying-components-by-sha-values).

One can use this utility to lookup a version details of the package, fetch a license ID for the binary or 
get vulnerability details or automate due diligence process without installing any runtime or additional dependencies.

Default file extensions for package managers:

* Nuget (SHA512) - *\*.nupkg*
* Maven (SHA1)   - *\*.jar*
* PYPI  (MD5)    - *\*.tar.gz, \*.whl*
* NPM   (SHA1)   - *\*.tgz*

## Usage

Download binaries from the [releases](https://github.com/versioneye/veye-checker/releases) and save it into your binaries folder

```bash
#NB! change version and op-sys
curl -s -L -o "${HOME}/bin/veye_checker" https://github.com/versioneye/veye-checker/releases/download/v0.2.0/veye_checker_osx

chmod a+x ~/bin/veye_checker
```

* **resolve** - scans the target folder recursively, translates a value of a file digest via VersionEye API into the product details and prints out results.

```bash
veye_checker resolve ../jars -a "api-key" -c "confs/veye_checker_local.toml"
VERSIONEYE_API_KEY="apitoken" veye_checker resolve ../jars
veye_checker resolve ../jars -o resolve.csv -a "api-key"
```

configure which digest algorithms to use
commandline flags for blocking algos: `no-md5, no-sha1, no-sha512`
commandline options to overwrite list of file-extensions of a digest algos: `ext-md5, ext-sha1, ext-sha512`

```bash
veye_checker resolve ../jars -a "api-key" --no-md5 --ext-sha1="whl,jar,tgz"
```



* **shas** - scans the target folder recursively and outputs digests of supported packagefiles:

```bash
veye_checker shas ../jars/ 
veye_checker shas ../jars/ -o results.csv
VERSIONEYE_CSV_SEPARATOR="," veye_checker shas temp/bins/
```

It is possible to configure which digest algorithms to use.
commandline flags for blocking algos: `no-md5, no-sha1, no-sha512`
commandline options to overwrite list of file-extensions of a digest algos: `ext-md5, ext-sha1, ext-sha512`

```bash
# dont use MD5 for next scan and update file extensions to use for SHA1
veye_checker shas ../jars -a "api-key" --no-md5 --ext-sha1="whl,jar,tgz"
```



* **lookup** - fetches product details from VersionEye api by the SHA/digest value.

```bash
veye_checker lookup <SHA_STRING> -a <YOUR_API_KEY>

VERSIONEYE_API_KEY="apikey" veye_checker lookup <SHA_STRING>
```

## API keys

All the commands ( *lookup*, *resolve*, etc ) requesting data from the  [VersionEye api](https://www.versioneye.com/api/v2) require the API-key,
which you can obtain from [your's profile page](https://www.versioneye.com/organisations/private/apikey).

It's possible to specify the api-key 3 ways:

* via environment variable `VERSIONEYE_API_KEY` 

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
| VERSIONEYE\_CSV\_SEPARATOR| ;         | overrides separator in output row, can be only single character|
| VERSIONEYE\_CSV\_QUOTE | \"           | what character to use for quoting, can be only single character |
| VERSIONEYE\_CSV\_FLEXIBLE| false      | should it skip empty fields at the end, accepted values 1, T, TRUE to activate; all other values equal to FALSE |
| VERSIONEYE\_PROXY\_HOST| None         | specifies proxy host |
| VERSIONEYE\_PROXY\_PORT| None         | specifies proxy port |
| VERSIONEYE\_PROXY\_SCHEME| http       | specifies proxy scheme |

NB! Use cmd-line flags or config-file to configure file extensions used by a digest algo;

## Configuration via config file

One can also put all the permanent configurations for the `veye_checker` tool into a `veye_checker.toml` file.
By default the tool will lookup configuration file in the working directory, but you can always specify
location with the `-c` flag or `--config` option after the subcommand.

All the fields in the configuration file are optional, and the commandline tool will use default values for unspecified fields.

```toml
# veye_checker.toml
[api]
key = "Your API key"
host = "www.versioneye.com"
path = "api/v2"
port = 80
scheme = "https"

[csv]
separator = ","
quote     = "'"
flexible  = 0

[proxy]
host = "127.0.0.1"
port = 3128
scheme = "http"

# configure file extensions
[digests.md5]
blocked = false
exts = ["whl", "gz"]

# Dont use SHA1
[digests.sha1]
blocked = true

```

## Build

```bash
cargo build
./target/debug/veye_checker

# or simpler command
cargo run

# or running tests
cargo test -- --test-threads=1

# test only api-calls 
VERSIONEYE_API_KEY="APIKEY" cargo test --features "api"

# or optimized production release
cargo build --release
./target/release/veye-checker

```

### TESTING

* to run all the unit tests

```bash
cargo test -- --test-threads=1
```

`--test-threads=1` is required for tests that are checking does reading configuration from ENV variables work;

* to run integration test against API configs

```bash
VERSIONEYE_API_KEY="your_api_key" cargo test --features="api"
```

* running integration tests against proxy

 1. start squid proxy
 
```bash
docker pull sameersbn/squid:latest

docker run --name squid -d --restart=always \
  --publish 3128:3128 \
  --volume /veye-checker/temp/cache:/var/spool/squid3 \
  sameersbn/squid:latest
  
docker stop|run squid
```

 2. run tests
 
```bash
    cargo test test_proxy --features=proxy
```


* to run acceptance tests

```bash
cd tests/acceptance

# on *nix machines
VERSIONEYE_API_KEY="your_api_key" ./tests.sh 

# on Macs
VERSIONEYE_API_KEY="your_api_key" ./tests_osx.sh
```


## Contributing

It's opensource project and any kind of contribution is more than welcome. 

Here's simple guideline to preferable workflow:

* open a issue
* implement after it lands into milestones
* write tests
* update docs
* make PR
* review

and your changes makes into next release



