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
    let confs = configs::read_api_configs_from_env().expect("Failed to read configs from ENV");

    assert_eq!(confs.api.key,  Some("veye-123".to_string()) );
    assert_eq!(confs.api.host, Some("api.veye.com".to_string()) );
    assert_eq!(confs.api.path, Some("api/v2".to_string()) );
    assert_eq!(confs.api.port, Some(8080));
    assert_eq!(confs.api.scheme, Some("https".to_string()))
}

#[test]
fn read_configs_from_toml_test(){
    let toml_path = Path::new("tests/fixtures/veye_checker.toml");
    let confs = configs::read_configs_from_toml(toml_path).expect("Failed to parse test TOML");

    println!("Configs from toml...{:?}", confs );

    assert_eq!(confs.api.key, Some("def-234".to_string()));
    assert_eq!(confs.api.port, Some(8090));

}


#[test]
fn read_configs_test(){
    //set up env
    env::set_var("VERSIONEYE_API_KEY", "veye-123");
    env::set_var("VERSIONEYE_API_HOST", "api.veye.com");
    env::set_var("VERSIONEYE_API_PORT", "8080");

    //execute tests
    let confs = configs::read_configs();
    assert_eq!(confs.api.key, Some("veye-123".to_string()));
    assert_eq!(confs.api.host, Some("api.veye.com".to_string()));
    assert_eq!(confs.api.path, Some("api/v2".to_string()));
    assert_eq!(confs.api.port, Some(8080));
    assert_eq!(confs.api.scheme, Some("https".to_string()))
}

