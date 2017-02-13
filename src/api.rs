use std::io::{self, Read, Error, ErrorKind};

use hyper;
use hyper::{ Client, Url };
use hyper::client::Response;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use rustc_serialize::json::{self, ToJson, Json};

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
pub struct ProductDetail {
    language: String,
    prod_key: String,
    version: String,
    packaging: String,
    prod_type: String,
    sha_value: String,
    sha_method: String
}


pub trait CSVSerializer {
    fn to_csv(&self) -> String;
}

impl CSVSerializer for ProductDetail {
    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.sha_method, self.sha_value, self.language, self.prod_key, self.version
        )
    }
}

// todo: refactor it
fn process_sha_response(json_text: Option<String> ) -> Result<ProductDetail, io::Error> {
    let json_text = json_text.unwrap();
    let json_obj = Json::from_str( &json_text).expect("Failed to parse product JSON");

    if !json_obj.is_array() {
        println!("Found no matches");
        return Err(Error::new( ErrorKind::Other, "Product response wasnt array"));
    }

    let product_doc = json_obj[0].clone();
    let product = ProductDetail {
        language: product_doc["language"].as_string().unwrap().to_string(),
        prod_key: product_doc["prod_key"].as_string().unwrap().to_string(),
        version: product_doc["version"].as_string().unwrap().to_string(),
        packaging: product_doc["packaging"].as_string().unwrap().to_string(),
        prod_type: product_doc["prod_type"].as_string().unwrap().to_string(),
        sha_value: product_doc["sha_value"].as_string().unwrap().to_string(),
        sha_method: product_doc["sha_method"].as_string().unwrap().to_string()
    };

    println!("product_doc: {}", product_doc);

    Ok(product)
}

fn request_json(req_url: &str) -> Option<String> {

    let uri = Url::parse(req_url).ok().expect("malformed url");
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let mut res = client.get(uri).send().expect("Failed to fetch results from the url");

    let mut body = String::new();
    res.read_to_string(&mut body).expect("Failed to read response body");

    Some(body)
}

pub fn fetch_product_by_sha(sha: &str, api_key: &str) -> Option<ProductDetail> {

    let api_url = format!(
        "https://www.versioneye.com/api/v2/products/sha/{}?api_key={}",
        sha, api_key
    );

    let json_txt = request_json( &api_url );
    match process_sha_response(json_txt) {
        Ok(product) => Some(product),
        Err(err)        => None
    }
}
