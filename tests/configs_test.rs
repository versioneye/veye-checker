extern crate veye_checker;

use std::env;
use std::path::PathBuf;
use veye_checker::configs;

#[test]
fn test_configs_read_api_configs_from_env(){
    //set up env
    env::set_var("VERSIONEYE_API_KEY", "veye-123");
    env::set_var("VERSIONEYE_API_HOST", "api.veye.com");
    env::set_var("VERSIONEYE_API_PORT", "8080");
    env::remove_var("VERSIONEYE_CSV_SEPARATOR");

    //run tests
    let confs = configs::read_configs_from_env().expect("Failed to read configs from ENV");

    assert_eq!(confs.api.key,  Some("veye-123".to_string()) );
    assert_eq!(confs.api.host, Some("api.veye.com".to_string()) );
    assert_eq!(confs.api.path, Some("api/v2".to_string()) );
    assert_eq!(confs.api.port, Some(8080));
    assert_eq!(confs.api.scheme, Some("https".to_string()));

    //cleanup env
    env::remove_var("VERSIONEYE_API_KEY");
    env::remove_var("VERSIONEYE_API_HOST");
    env::remove_var("VERSIONEYE_API_PORT");
    //cleanup some ENV vars to get values from config file
    env::remove_var("VERSIONEYE_CSV_SEPARATOR");
    println!("IS key removed? {:?}", env::var("VERSIONEYE_CSV_SEPARATOR").is_err());
    assert!(env::var("VERSIONEYE_CSV_SEPARATOR").is_err());
}

#[test]
fn test_configs_read_csv_configs_from_env(){
    //set up env
    env::set_var("VERSIONEYE_CSV_SEPARATOR", ",");
    env::set_var("VERSIONEYE_CSV_QUOTE", "'");
    env::set_var("VERSIONEYE_CSV_FLEXIBLE", "1");

    //test correctness
    let confs = configs::read_configs_from_env().expect("Failed to read CSV configs from ENV");

    assert_eq!(Some(",".to_string()), confs.csv.separator);
    assert_eq!(Some("'".to_string()), confs.csv.quote);
    assert_eq!(Some(true), confs.csv.flexible);

    //cleanup env
    env::set_var("VERSIONEYE_CSV_FLEXIBLE", "0");
    env::remove_var("VERSIONEYE_CSV_QUOTE");
    //cleanup some ENV vars to get values from config file
    env::remove_var("VERSIONEYE_CSV_SEPARATOR");
    println!("IS key removed? {:?}", env::var("VERSIONEYE_CSV_SEPARATOR").is_err());
    assert!(env::var("VERSIONEYE_CSV_SEPARATOR").is_err());
}

#[test]
fn test_configs_read_proxy_configs_from_env(){
    //set up env variables
    env::set_var("VERSIONEYE_PROXY_HOST", "127.0.0.1");
    env::set_var("VERSIONEYE_PROXY_PORT", "3128");
    env::set_var("VERSIONEYE_PROXY_SCHEME", "socks");

    //test correctness
    let confs = configs::read_configs_from_env().expect("Failed to read configs from ENV");
    assert_eq!(Some("127.0.0.1".to_string()), confs.proxy.host);
    assert_eq!(Some(3128), confs.proxy.port);
    assert_eq!(Some("socks".to_string()), confs.proxy.scheme);

    //cleanup env
    env::remove_var("VERSIONEYE_PROXY_HOST");
    env::remove_var("VERSIONEYE_PROXY_PORT");
    env::remove_var("VERSIONEYE_PROXY_SCHEME");
}

#[test]
fn test_configs_read_configs_from_toml(){

    let toml_path = PathBuf::from("./tests/fixtures/veye_checker.toml");
    let confs = configs::read_configs_from_toml(&toml_path).expect("Failed to parse test TOML");

    assert_eq!(confs.api.key, Some("def-234".to_string()));
    assert_eq!(confs.api.port, Some(8090));

    //check correctness of CSV configs
    assert_eq!(Some(",".to_string()), confs.csv.separator);
    assert_eq!(Some("'".to_string()), confs.csv.quote);
    assert_eq!(Some(false), confs.csv.flexible);

    //check correctness of proxy settings
    assert_eq!(Some("192.168.0.1".to_string()), confs.proxy.host);
    assert_eq!(Some(9200), confs.proxy.port);
    assert_eq!(None, confs.proxy.scheme);

    //cleanup some ENV vars to get values from config file
    env::remove_var("VERSIONEYE_CSV_SEPARATOR");
    assert!(env::var("VERSIONEYE_CSV_SEPARATOR").is_err());

}

#[test]
fn test_configs_read_toml_file_only_api_configs(){
    let toml_path = PathBuf::from("./tests/fixtures/only_api.toml");
    let confs = configs::read_configs_from_toml(&toml_path).expect("Failed to parse `only_api.toml`");

    //specified fields
    assert_eq!(confs.api.host, Some("only.api.com".to_string()));
    assert_eq!(confs.api.path, Some("api/v4".to_string()));
    assert_eq!(confs.api.port, Some(8010));

    //unspecified fields
    assert_eq!(confs.api.key, None);
    assert_eq!(confs.api.scheme, None);

}

#[test]
fn test_configs_read_toml_file_only_csv_configs(){
    let toml_path = PathBuf::from("./tests/fixtures/only_csv.toml");
    let confs = configs::read_configs_from_toml(&toml_path).expect("Failed to parse `only_csv.toml`");

    //specified fields
    assert_eq!(confs.csv.separator, Some(",".to_string()));

    //unspecified fields
    assert_eq!(confs.csv.flexible, None);
    assert_eq!(confs.csv.quote, None);

}

#[test]
fn test_configs_read_toml_file_only_proxy_configs(){
    let toml_path = PathBuf::from("./tests/fixtures/only_proxy.toml");
    let confs = configs::read_configs_from_toml(&toml_path).expect("Failed to parse `only_csv.toml`");

    //specified fields
    assert_eq!(confs.proxy.host, Some("192.168.2.1".to_string()));

    //unspecified fields
    assert_eq!(confs.proxy.port, None);

}

#[test]
fn test_configs_read_toml_file(){
    //set up env
    env::set_var("VERSIONEYE_API_KEY", "veye-123");
    env::set_var("VERSIONEYE_API_HOST", "api.veye.com");
    env::set_var("VERSIONEYE_API_PORT", "8080");
    env::set_var("VERSIONEYE_CSV_FLEXIBLE", "T");

    //cleanup some ENV vars to get values from config file
    env::remove_var("VERSIONEYE_CSV_QUOTE");
    env::remove_var("VERSIONEYE_CSV_SEPARATOR");
    env::remove_var("VERSIONEYE_CSV_FLEXIBLE");
    println!("IS key removed? {:?}", env::var("VERSIONEYE_CSV_SEPARATOR").is_err());
    assert!(env::var("VERSIONEYE_CSV_SEPARATOR").is_err());

    //execute tests
    let conf_filepath = "./tests/fixtures/veye_checker.toml";
    let confs = configs::read_configs(Some(conf_filepath.to_string()));

    assert_eq!(confs.api.key, Some("veye-123".to_string()));
    assert_eq!(confs.api.host, Some("api.veye.com".to_string()));
    assert_eq!(confs.api.path, Some("api/v2".to_string()));
    assert_eq!(confs.api.port, Some(8080));
    assert_eq!(confs.api.scheme, Some("https".to_string()));

    //TODO: it has read-write conflicts when multiple test add&remove VARs at the same time
    // due the that fact, it uses old ENV vars instead data from configfile
    //CSV values should come from config file
    //assert_eq!(Some(",".to_string()), confs.csv.separator);
    //assert_eq!(Some("'".to_string()), confs.csv.quote);
    //assert_eq!(Some(true), confs.csv.flexible);

    //cleanup env
    env::remove_var("VERSIONEYE_API_KEY");
    env::remove_var("VERSIONEYE_API_HOST");
    env::remove_var("VERSIONEYE_API_PORT");
    env::remove_var("VERSIONEYE_CSV_QUOTE");
    env::remove_var("VERSIONEYE_CSV_SEPARATOR");
    env::remove_var("VERSIONEYE_CSV_FLEXIBLE");
}




