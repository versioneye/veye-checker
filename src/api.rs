use std::io::{self, Read, Error, ErrorKind};

use hyper;
use hyper::{ Client, Url };
use hyper::client::Response;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::time::Duration;

use rustc_serialize::json::{self, ToJson, Json};
use product;

const API_URL: &'static str = "https://www.versioneye.com/api/v2";
const HOST_URL: &'static str = "https://www.versioneye.com";

fn request_json(req_url: &str) -> Option<String> {

    let uri = Url::parse(req_url).ok().expect("malformed url");
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let mut client = Client::with_connector(connector);
    client.set_read_timeout(Some(Duration::new(5,0)));

    let mut res = client.get(uri).send().expect("Failed to fetch results from the url");

    let mut body = String::new();
    res.read_to_string(&mut body).expect("Failed to read response body");

    Some(body)
}

pub fn fetch_product_by_sha(sha: &str, api_key: &str) -> Result<product::ProductMatch, io::Error> {
    let resource_url = format!("{}/products/sha/{}?api_key={}", API_URL, sha, api_key);

    let json_txt = request_json( &resource_url );
    println!("Processing sha response....");
    process_sha_response(json_txt)
}

pub fn fetch_product(
    lang: &str, prod_key: &str, version: &str, api_key: &str
) -> Result<product::ProductMatch, io::Error> {

    let prod_key = str::replace(prod_key, "/", ":");
    let prod_key = str::replace(&prod_key, ".", "~");
    let resource_url = format!(
        "{}/products/{}/{}?prod_version={}&api_key={}",
        API_URL, lang, prod_key, version, api_key
    );
    let prod_url = format!("{}/{}/{}/{}", HOST_URL, lang, prod_key, version);

    let json_txt = request_json( &resource_url );
    process_product_response(json_txt, Some(prod_url))
}


//-- helper functions
fn process_sha_response(json_text: Option<String> ) -> Result<product::ProductMatch, io::Error> {
    let json_text = json_text.expect("process_sha_response: got null json text");
    let json_obj = Json::from_str( &json_text).expect("Failed to parse product JSON");

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
        packaging: product_doc["packaging"].as_string().expect("No field `packaging`").to_string(),
        method: product_doc["sha_method"].as_string().expect("No field `sha_method`").to_string(),
        value: product_doc["sha_value"].as_string().expect("No field `sha_value`").to_string()
    };

    Ok(product::ProductMatch::new(the_prod, the_sha))
}

fn process_product_response(
    json_text: Option<String>, prod_url: Option<String>
) -> Result<product::ProductMatch, io::Error> {

    let json_text = json_text.expect("process_product_response: got none JSON doc");
    let json_obj = Json::from_str( &json_text).expect("Failed to parse product JSON");

    if !json_obj.is_object() {
        return Err(Error::new(ErrorKind::Other, "Response had new product details"));
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
                url: x["url"].as_string().unwrap().to_string()
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
        filepath: None
    };

    Ok(the_match)
}
