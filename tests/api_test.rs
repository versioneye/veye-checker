extern crate hyper;
extern crate veye_checker;

use std::env;
use std::error::Error;
use veye_checker::{api, configs};


#[test]
fn test_api_encode_prod_key(){
    assert_eq!(api::encode_prod_key ("dot.net"), "dot~net");
    assert_eq!(api::encode_prod_key("slash/net"), "slash:net");
    assert_eq!(api::encode_prod_key("dot.net/slash"), "dot~net:slash");
}

#[test]
fn test_api_encode_language(){
    assert_eq!("csharp".to_string(), api::encode_language("CSharp"));
    assert_eq!("nodejs".to_string(), api::encode_language("Node.JS"));
    assert_eq!("java".to_string(), api::encode_language("java"));
}

#[test]
fn test_api_encode_sha(){
    let original_sha = "/uVunD0tcI2UQCzFp0g46+CjSF2ElQ/Bc9tWw8FS4f0iIK728XCjY8stn3+s78tiz8x2EUGwAnaOVfQJB6hI5g==";
    let encoded_sha = "%2FuVunD0tcI2UQCzFp0g46%2BCjSF2ElQ%2FBc9tWw8FS4f0iIK728XCjY8stn3%2Bs78tiz8x2EUGwAnaOVfQJB6hI5g%3D%3D";
    assert_eq!(encoded_sha.to_string(), api::encode_sha(original_sha));
}

#[test]
#[cfg(feature="api")]
fn test_api_call_fetch_product_by_sha(){

    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let confs = configs::read_configs(None);

    let res = api::fetch_product_by_sha(&confs, file_sha).expect("Failed fetch SHA");

    let prod_url = "https://www.versioneye.com/Java/commons-beanutils/commons-beanutils".to_string();
    assert_eq!(Some(prod_url), res.url);
    assert_eq!(true, res.sha.is_some());

    let sha = res.sha.unwrap();
    assert_eq!("jar".to_string(), sha.packaging);
    assert_eq!("sha1".to_string(), sha.method);
    assert_eq!(file_sha.to_string(), sha.value);
    assert_eq!(None, sha.filepath);

    assert_eq!(true, res.product.is_some());
    let prod = res.product.unwrap();
    assert_eq!("Java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("".to_string(), prod.name);
}

#[test]
#[cfg(feature="api")]
fn test_api_call_fetch_product_by_sha_nuget_special_symbols(){

    let file_sha = "/uVunD0tcI2UQCzFp0g46+CjSF2ElQ/Bc9tWw8FS4f0iIK728XCjY8stn3+s78tiz8x2EUGwAnaOVfQJB6hI5g==";
    let confs = configs::read_configs(None);

    let res = api::fetch_product_by_sha(&confs, file_sha).expect("Failed fetch SHA");

    let prod_url = "https://www.versioneye.com/CSharp/RavenDB.Client".to_string();
    assert_eq!(Some(prod_url), res.url);
    assert_eq!(true, res.sha.is_some());

    let sha = res.sha.unwrap();
    assert_eq!("unknown".to_string(), sha.packaging);
    assert_eq!("sha512".to_string(), sha.method);
    assert_eq!(file_sha.to_string(), sha.value);
    assert_eq!(None, sha.filepath);

    assert_eq!(true, res.product.is_some());
    let prod = res.product.unwrap();
    assert_eq!("CSharp".to_string(), prod.language);
    assert_eq!("Nuget".to_string(), prod.prod_type.unwrap());
    assert_eq!("RavenDB.Client".to_string(), prod.prod_key);
    assert_eq!("3.5.2".to_string(), prod.version);
    assert_eq!("".to_string(), prod.name);
}

//1.st run Squid container, 2. cargo test test_proxy --features=proxy
#[test]
#[cfg(feature="proxy")]
fn test_proxy_call_fetch_product_by_sha(){
    env::set_var("VERSIONEYE_PROXY_HOST", "127.0.0.1");
    env::set_var("VERSIONEYE_PROXY_PORT", "3128");
    env::set_var("VERSIONEYE_PROXY_SCHEME", "http");

    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let confs = configs::read_configs(None);

    let res = api::fetch_product_by_sha(&confs, file_sha).expect("Failed fetch SHA");

    let prod_url = "https://www.versioneye.com/Java/commons-beanutils/commons-beanutils".to_string();
    assert_eq!(Some(prod_url), res.url);
    assert_eq!(true, res.sha.is_some());

    let sha = res.sha.unwrap();
    assert_eq!("jar".to_string(), sha.packaging);
    assert_eq!("sha1".to_string(), sha.method);
    assert_eq!(file_sha.to_string(), sha.value);
    assert_eq!(None, sha.filepath);

    assert_eq!(true, res.product.is_some());
    let prod = res.product.unwrap();
    assert_eq!("Java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("".to_string(), prod.name);

    //clean up env
    env::remove_var("VERSIONEYE_PROXY_HOST");
    env::remove_var("VERSIONEYE_PROXY_PORT");
    env::remove_var("VERSIONEYE_PROXY_SCHEME");
}

#[test]
#[cfg(feature="api")]
fn test_api_call_fetch_product(){
    let confs = configs::read_configs(None);
    let res = api::fetch_product(
        &confs, "Java", "commons-beanutils/commons-beanutils", "1.7.0"
    ).expect("Failed to fetch product details");

    assert_eq!(false, res.sha.is_some());
    assert_eq!(true, res.product.is_some());

    let prod = res.product.unwrap();
    assert_eq!("java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("commons-beanutils".to_string(), prod.name);
}

#[test]
#[cfg(feature="proxy")]
fn test_proxy_call_fetch_product(){
    env::set_var("VERSIONEYE_PROXY_HOST", "127.0.0.1");
    env::set_var("VERSIONEYE_PROXY_PORT", "3128");
    env::set_var("VERSIONEYE_PROXY_SCHEME", "http");

    let confs = configs::read_configs(None);
    let res = api::fetch_product(
        &confs, "Java", "commons-beanutils/commons-beanutils", "1.7.0"
    ).expect("Failed to fetch product details");

    assert_eq!(false, res.sha.is_some());
    assert_eq!(true, res.product.is_some());

    let prod = res.product.unwrap();
    assert_eq!("java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("commons-beanutils".to_string(), prod.name);

    //clean up env
    env::remove_var("VERSIONEYE_PROXY_HOST");
    env::remove_var("VERSIONEYE_PROXY_PORT");
    env::remove_var("VERSIONEYE_PROXY_SCHEME");
}


#[test]
#[cfg(feature="api")]
fn test_api_call_fetch_product_details_by_sha(){
    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let confs = configs::read_configs(None);

    let res = api::fetch_product_by_sha(&confs, file_sha).expect("Failed fetch SHA");

    let prod_url = "https://www.versioneye.com/Java/commons-beanutils/commons-beanutils".to_string();
    assert_eq!(Some(prod_url), res.url);
    assert_eq!(true, res.sha.is_some());

    let sha = res.sha.unwrap();
    assert_eq!("jar".to_string(), sha.packaging);
    assert_eq!("sha1".to_string(), sha.method);
    assert_eq!(file_sha.to_string(), sha.value);
    assert_eq!(None, sha.filepath);

    assert_eq!(true, res.product.is_some());
    let prod = res.product.unwrap();
    assert_eq!("Java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("".to_string(), prod.name);
}

#[test]
#[cfg(feature="proxy")]
fn test_proxy_call_fetch_product_details_by_sha(){
    env::set_var("VERSIONEYE_PROXY_HOST", "127.0.0.1");
    env::set_var("VERSIONEYE_PROXY_PORT", "3128");
    env::set_var("VERSIONEYE_PROXY_SCHEME", "http");

    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let confs = configs::read_configs(None);

    let res = api::fetch_product_by_sha(&confs, file_sha).expect("Failed fetch SHA");

    let prod_url = "https://www.versioneye.com/Java/commons-beanutils/commons-beanutils".to_string();
    assert_eq!(Some(prod_url), res.url);
    assert_eq!(true, res.sha.is_some());

    let sha = res.sha.unwrap();
    assert_eq!("jar".to_string(), sha.packaging);
    assert_eq!("sha1".to_string(), sha.method);
    assert_eq!(file_sha.to_string(), sha.value);
    assert_eq!(None, sha.filepath);

    assert_eq!(true, res.product.is_some());
    let prod = res.product.unwrap();
    assert_eq!("Java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("".to_string(), prod.name);

    //clean up env
    env::remove_var("VERSIONEYE_PROXY_HOST");
    env::remove_var("VERSIONEYE_PROXY_PORT");
    env::remove_var("VERSIONEYE_PROXY_SCHEME");
}

#[test]
fn test_api_process_sha_response(){
    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let res_body = r#"
    [{
        "language":"Java",
        "prod_key":"commons-beanutils/commons-beanutils",
        "version":"1.7.0",
        "group_id":"commons-beanutils",
        "artifact_id":"commons-beanutils",
        "classifier":null,"packaging":"jar",
        "prod_type":"Maven2",
        "sha_value":"5675fd96b29656504b86029551973d60fb41339b",
        "sha_method":"sha1"
    }]
    "#;

    let res = api::process_sha_response(Some(res_body.to_string()));

    if let Some(prod_match) = res.ok() {

        assert_eq!(true, prod_match.sha.is_some());
        let sha = prod_match.sha.unwrap();
        assert_eq!("jar".to_string(), sha.packaging);
        assert_eq!("sha1".to_string(), sha.method);
        assert_eq!(file_sha.to_string(), sha.value);
        assert_eq!(None, sha.filepath);

        assert_eq!(true, prod_match.product.is_some());
        let prod = prod_match.product.unwrap();
        assert_eq!("Java".to_string(), prod.language);
        assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
        assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
        assert_eq!("1.7.0".to_string(), prod.version);
        assert_eq!("".to_string(), prod.name);
    } else {
        assert_eq!("", "Failed to process sha response");
    }
}

#[test]
fn test_api_process_sha_response_with_null_fields(){
    let file_sha = "U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==";
    let res_body = r#"
    [
        {
            "language": "CSharp",
            "prod_key": "Newtonsoft.Json",
            "version": "9.0.1",
            "group_id": null,
            "artifact_id": null,
            "classifier": null,
            "packaging": null,
            "prod_type": "Nuget",
            "sha_value": "U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==",
            "sha_method": "sha512"
        }
    ]
    "#;

    let res = api::process_sha_response(Some(res_body.to_string()));
    assert_eq!(true, res.is_ok());

    if let Some(prod_match) = res.ok() {
        assert_eq!(true, prod_match.sha.is_some());
        let sha = prod_match.sha.unwrap();
        assert_eq!("unknown".to_string(), sha.packaging);
        assert_eq!("sha512".to_string(), sha.method);
        assert_eq!(file_sha.to_string(), sha.value);
        assert_eq!(None, sha.filepath);

        assert_eq!(true, prod_match.product.is_some());
        let prod = prod_match.product.unwrap();
        assert_eq!("CSharp".to_string(), prod.language);
        assert_eq!("Nuget".to_string(), prod.prod_type.unwrap());
        assert_eq!("Newtonsoft.Json".to_string(), prod.prod_key);
        assert_eq!("9.0.1".to_string(), prod.version);
        assert_eq!("".to_string(), prod.name);
    }

}

#[test]
fn test_api_process_sha_response_for_npm(){
    let file_sha = "6f631aef336d6c46362b51764044ce216be3c051";
    let res_body = r#"
    [{
        "language":"Node.JS",
        "prod_key":"etag",
        "version":"1.8.0",
        "group_id":null,
        "artifact_id":null,
        "classifier":null,
        "packaging":null,
        "prod_type":"npm",
        "sha_value":"6f631aef336d6c46362b51764044ce216be3c051",
        "sha_method":"sha1"
     }]
    "#;

    let res = api::process_sha_response(Some(res_body.to_string()));
    assert!(res.is_ok());

    if let Some(prod_match) = res.ok() {
        assert!(prod_match.sha.is_some());
        let sha = prod_match.sha.unwrap();
        assert_eq!("unknown".to_string(), sha.packaging);
        assert_eq!("sha1".to_string(), sha.method);
        assert_eq!(file_sha.to_string(), sha.value);
        assert_eq!(None, sha.filepath);

        assert!(prod_match.product.is_some());
        let prod = prod_match.product.unwrap();
        assert_eq!("Node.JS".to_string(), prod.language);
        assert_eq!("npm".to_string(), prod.prod_type.unwrap());
        assert_eq!("etag".to_string(), prod.prod_key);
        assert_eq!("1.8.0".to_string(), prod.version);
        assert_eq!("".to_string(), prod.name);

    }
}

#[test]
fn test_api_process_sha_response_with_empty_result(){
    let res = api::process_sha_response(Some("".to_string()));
    assert_eq!(true, res.is_err());
    let e = res.err().unwrap();
    println!("message: {}", e.description());
}

#[test]
fn test_api_process_sha_response_with_api_error(){
    let body_txt = r#"
        {"error": "Failed to match it"}
    "#;
    let res = api::process_sha_response(Some(body_txt.to_string()));
    assert_eq!(true, res.is_err());
}

#[test]
fn test_api_process_product_response(){
    let body_txt = r#"
    {
        "name": "commons-beanutils",
        "language": "java",
        "prod_key": "commons-beanutils/commons-beanutils",
        "version": "1.7.0",
        "prod_type": "Maven2",
        "group_id": "commons-beanutils",
        "artifact_id": "commons-beanutils",
        "license_info": "unknown",
        "description": "BeanUtils provides ...",
        "licenses" : [],
        "security_vulnerabilities" : [
            {"id":  1},
            {"id" : 2}
        ]
  }
    "#;

    let res = api::process_product_response(Some(body_txt.to_string()), None);
    assert_eq!(true, res.is_ok());
    let prod_match = res.unwrap();
    assert_eq!(0, prod_match.licenses.len());
    assert_eq!(2, prod_match.n_vulns);

    assert_eq!(true, prod_match.product.is_some());
    let prod = prod_match.product.unwrap();
    assert_eq!("java".to_string(), prod.language);
    assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
    assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
    assert_eq!("1.7.0".to_string(), prod.version);
    assert_eq!("commons-beanutils".to_string(), prod.name);

}

#[test]
fn test_api_process_product_response_with_empty_result(){
    let res = api::process_product_response(Some("".to_string()), None);
    assert_eq!(true, res.is_err());
    let e = res.err().unwrap();
    println!("message: {}", e.description());
}

#[test]
fn test_api_process_product_response_with_api_error(){
    let body_txt = r#"
        {"error": "Failed to match it"}
    "#;
    let res = api::process_product_response(Some(body_txt.to_string()), None);
    assert_eq!(true, res.is_err());
}