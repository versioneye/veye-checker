extern crate veye_checker;

use std::env;
use std::path::Path;
use veye_checker::configs;

#[test]
fn read_api_configs_from_env_test(){
    //set up env
    env::set_var("VERSIONEYE_API_KEY", "veye-123");
    env::set_var("VERSIONEYE_API_HOST", "api.veye.com");
    env::set_var("VERSIONEYE_API_PORT", "8080");
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
}

#[test]
fn read_csv_configs_from_env_test(){
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
    env::remove_var("VERSIONEYE_CSV_SEPARATOR");
    env::remove_var("VERSIONEYE_CSV_QUOTE");
}


#[test]
fn read_configs_from_toml_test(){
    let toml_path = Path::new("tests/fixtures/veye_checker.toml");
    let confs = configs::read_configs_from_toml(toml_path).expect("Failed to parse test TOML");

    println!("Configs from toml...{:?}", confs );

    assert_eq!(confs.api.key, Some("def-234".to_string()));
    assert_eq!(confs.api.port, Some(8090));

    //check correctness of CSV configs
    assert_eq!(Some(",".to_string()), confs.csv.separator);
    assert_eq!(Some("'".to_string()), confs.csv.quote);
    assert_eq!(Some(false), confs.csv.flexible);

}


#[test]
fn read_configs_test(){
    //set up env
    env::set_var("VERSIONEYE_API_KEY", "veye-123");
    env::set_var("VERSIONEYE_API_HOST", "api.veye.com");
    env::set_var("VERSIONEYE_API_PORT", "8080");
    env::set_var("VERSIONEYE_CSV_FLEXIBLE", "T");

    //execute tests
    let confs = configs::read_configs();
    assert_eq!(confs.api.key, Some("veye-123".to_string()));
    assert_eq!(confs.api.host, Some("api.veye.com".to_string()));
    assert_eq!(confs.api.path, Some("api/v2".to_string()));
    assert_eq!(confs.api.port, Some(8080));
    assert_eq!(confs.api.scheme, Some("https".to_string()));

    //test CSV values
    assert_eq!(Some(",".to_string()), confs.csv.separator);
    assert_eq!(Some("'".to_string()), confs.csv.quote);
    assert_eq!(Some(true), confs.csv.flexible);

    //cleanup env
    env::remove_var("VERSIONEYE_API_KEY");
    env::remove_var("VERSIONEYE_API_HOST");
    env::remove_var("VERSIONEYE_API_PORT");
    env::remove_var("VERSIONEYE_CSV_FLEXIBLE");
}

