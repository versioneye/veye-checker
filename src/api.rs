use std::io::{self, Read, Error, ErrorKind};
use std::borrow::Cow;

use hyper;
use hyper::{client, Client, Url };
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use std::time::Duration;
use rustc_serialize::json::Json;
use product;
use configs::{Configs, ApiConfigs, ProxyConfigs};

const HOST_URL: &'static str = "https://www.versioneye.com";

fn to_product_url(lang: &str, prod_key: &str, version: &str) -> String {
    format!("{}/{}/{}/{}", HOST_URL, lang, prod_key, version)
}

fn configs_to_url(api_confs: &ApiConfigs, resource_path: &str)
    -> Result<hyper::Url, hyper::error::ParseError> {
    let url_str = match api_confs.port {
        None => {
            format!(
                "{}://{}/{}/{}",
                api_confs.scheme.clone().unwrap(), api_confs.host.clone().unwrap(),
                api_confs.path.clone().unwrap(), resource_path,
            )

        },
        Some(port) => format!(
            "{}://{}:{}/{}/{}",
            api_confs.scheme.clone().unwrap(), api_confs.host.clone().unwrap(),
            port, api_confs.path.clone().unwrap(), resource_path
        )
    };

    Url::parse(url_str.as_str())
}

fn request_json<'a>(uri: &Url, proxy_confs: &'a ProxyConfigs) -> Option<String> {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);

    //use proxy only iff user has defined proxy host and port
    let mut client = if proxy_confs.is_complete() {
        let host = Cow::from(proxy_confs.host.clone().unwrap());
        let port = proxy_confs.port.clone().unwrap();
        let scheme = proxy_confs.scheme.clone().unwrap_or("http".to_string());

        let ssl_proxy = NativeTlsClient::new().unwrap();
        let proxy = client::ProxyConfig::new (
            scheme.as_str(), host, port, connector, ssl_proxy
        );

        Client::with_proxy_config(proxy)
    } else {
        Client::with_connector(connector)
    };

    client.set_read_timeout(Some(Duration::new(5,0)));

    let mut res = client.get(uri.as_str()).send().expect("Failed to fetch results from the url");
    let mut body = String::new();
    res.read_to_string(&mut body).expect("Failed to read response body");

    Some(body)
}

pub fn fetch_product_details_by_sha(confs: &Configs, file_sha: &str)
    -> Result<product::ProductMatch, Error> {

    let sha_res = fetch_product_by_sha(&confs, &file_sha);
    match sha_res {
        Ok(m) => {
            let sha = m.sha.expect("No product sha from SHA result");
            let product = m.product.expect("No product info from SHA result");
            match fetch_product( &confs, &product.language, &product.prod_key, &product.version ) {
                Ok(mut m) => {
                    m.sha = Some(sha);
                    Ok(m)
                },
                Err(e) => {
                    println!("Failed to fetch product details for sha: {}", file_sha);
                    Err(e)
                }
            }

        },
        Err(e) => Err(e)
    }
}

pub fn fetch_product_by_sha(confs: &Configs, sha: &str)
    -> Result<product::ProductMatch, io::Error> {
    let api_confs = confs.api.clone();
    let resource_path = format!("products/sha/{}", sha.clone() );
    let mut resource_url = match configs_to_url(&api_confs, resource_path.as_str()) {
        Ok(the_url) => the_url,
        Err(_)      => {
            return Err(
                Error::new(
                    ErrorKind::InvalidData,
                    "The values of API configs make up non-valid URL"
                )
            )
        }
    };

    //attach query params
    resource_url
        .query_pairs_mut()
        .clear()
        .append_pair("api_key", api_confs.key.clone().unwrap().as_str());


    let json_txt = request_json( &resource_url, &confs.proxy );
    process_sha_response(json_txt)
}

pub fn encode_prod_key<'b>(prod_key: &'b str) -> String {
    let encoded_prod_key = prod_key.to_string();
    encoded_prod_key
        .replace(".", "~")
        .replace("/", ":")
        .trim().to_string()

}

pub fn encode_language<'b>(lang: &'b str) -> String {
    let encoded_lang = lang.to_string();
    encoded_lang.replace(".", "").trim().to_lowercase().to_string()
}

pub fn fetch_product<'a>(
    confs: &Configs, lang: &str, prod_key: &str, version: &str
) -> Result<product::ProductMatch, io::Error> {

    let api_confs = confs.api.clone();
    let encoded_prod_key = encode_prod_key(&prod_key);
    let encoded_lang = encode_language(lang);
    let resource_path = format!("products/{}/{}", encoded_lang.clone(), encoded_prod_key.clone());
    let prod_url = to_product_url(encoded_lang.clone().as_str(), encoded_prod_key.clone().as_str(), version);

    let mut resource_url = match configs_to_url(&api_confs, resource_path.as_str()) {
        Ok(the_url) => the_url,
        Err(_)      => {
            return Err(
                Error::new(
                    ErrorKind::InvalidData,
                    "The values of API configs make up non-valid URL"
                )
            )
        }
    };

    //attach query params
    resource_url
        .query_pairs_mut()
        .clear()
        .append_pair("prod_version", version)
        .append_pair("api_key", api_confs.key.clone().unwrap().as_str());

    let json_txt = request_json( &resource_url, &confs.proxy );
    process_product_response(json_txt, Some(prod_url))
}


//-- helper functions
pub fn process_sha_response(json_text: Option<String> ) -> Result<product::ProductMatch, io::Error> {
    if json_text.is_none() {
        return Err(
            Error::new(ErrorKind::Other, "API returned empty response string")
        )
    }

    let json_res = Json::from_str( &json_text.clone().unwrap());
    if json_res.is_err() {
        return Err(
            Error::new(
                ErrorKind::Other,
                format!("Failed to parse JSON response from SHA api: {:?}", json_text).as_str()
            )
        )
    }

    let json_obj = json_res.unwrap();
    //if response includes error field in HTTP200 response
    if let Some(error_val) = json_obj.find("error") {
        return Err(
            Error::new(
                ErrorKind::Other,
                format!("Failed to process sha response: {}", error_val.as_string().unwrap())
            )
        )
    }

    if !json_obj.is_array() {
        return Err(Error::new( ErrorKind::Other, "Product response wasnt array"));
    }

    let product_doc = match json_obj.as_array() {
        Some(products) if products.len() > 0 => products[0].clone(),
        _ => return Err(Error::new(ErrorKind::Other, "Empty response"))
    };

    let the_prod = product::Product {
        name: "".to_string(),
        language: product_doc["language"].as_string().expect("No field `language`").to_string(),
        prod_key: product_doc["prod_key"].as_string().expect("No field `prod_key`").to_string(),
        version: product_doc["version"].as_string().expect("No field `version`").to_string(),
        prod_type: Some( product_doc["prod_type"].as_string().expect("No field `prod_types`").to_string() )
    };

    let the_sha = product::ProductSHA {
        packaging: product_doc["packaging"].as_string().unwrap_or("unknown").to_string(),
        method: product_doc["sha_method"].as_string().expect("No field `sha_method`").to_string(),
        value: product_doc["sha_value"].as_string().expect("No field `sha_value`").to_string(),
        filepath: None
    };

    Ok(product::ProductMatch::new(the_prod, the_sha))
}

// converts the response of product endpoint into ProductMatch struct
pub fn process_product_response(
    json_text: Option<String>, prod_url: Option<String>
) -> Result<product::ProductMatch, io::Error> {

    if json_text.is_none() {
        return Err(
            Error::new(ErrorKind::Other, "API returned empty response string")
        )
    }

    let json_res = Json::from_str( &json_text.clone().unwrap());
    if json_res.is_err() {
        return Err(
            Error::new(
                ErrorKind::Other,
                format!("Failed to parse JSON response from SHA api: {:?}", json_text).as_str()
            )
        )
    }

    let json_obj = json_res.unwrap();
    if !json_obj.is_object() {
        return Err(Error::new(ErrorKind::Other, "Response had no product details"));
    }

    //if response includes error field in HTTP200 response
    if let Some(error_val) = json_obj.find("error") {
        return Err(
            Error::new(
                ErrorKind::Other,
                format!("Failed to process product response: {}", error_val.as_string().unwrap())
            )
        )
    }

    let product_doc = json_obj.as_object().expect("Failed to fetch product document");
    let the_prod = product::Product {
        name: product_doc["name"].as_string().unwrap().to_string(),
        language: product_doc["language"].as_string().unwrap().to_string(),
        prod_key: product_doc["prod_key"].as_string().unwrap().to_string(),
        version: product_doc["version"].as_string().unwrap().to_string(),
        prod_type: Some( product_doc["prod_type"].as_string().unwrap_or("").to_string() )
    };

    let licenses = match product_doc["licenses"].as_array() {
        Some(arr) => arr.iter().fold(vec![], |mut acc, ref x| {
            acc.push(product::ProductLicense {
                name: x["name"].as_string().unwrap().to_string(),
                url: x["url"].as_string().unwrap_or("").to_string()
            });

            acc
        }),
        None      => vec![]
    };

    let n_vulns = match product_doc["security_vulnerabilities"].as_array() {
        Some(arr) => arr.len() as u32,
        None      => 0 as u32
    };

    let the_match = product::ProductMatch {
        sha: None,
        product: Some(the_prod),
        url: prod_url,
        licenses : licenses,
        n_vulns : n_vulns,
        error: None
    };

    Ok(the_match)
}
