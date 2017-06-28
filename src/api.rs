use std::str;
use std::io::{self, Read, Write, Error, ErrorKind};
use std::borrow::Cow;

use futures::{Future, Stream};
use hyper;
use hyper::{client, Client, Uri };
use hyper_tls::HttpsConnector;
use tokio_core::reactor;

use std::time::Duration;
use serde_json;
use product;
use configs::{Configs, ApiConfigs, ProxyConfigs};

const HOST_URL: &'static str = "https://www.versioneye.com";

//it is used to build url to the product page (SAAS or Enterprise)
pub fn to_product_url(api_confs: &ApiConfigs, lang: &str, prod_key: &str, version: &str) -> String {
    let scheme = match api_confs.scheme.clone() {
        Some(val)   => val,
        None        => "http".to_string()
    };
    let host = match api_confs.host.clone() {
        Some(val)   => val,
        None        => HOST_URL.to_string()
    };

    let host_url = match api_confs.port.clone() {
        Some(port)  => format!("{}://{}:{}", scheme, host, port),
        None        => format!("{}://{}", scheme, host )
    };

    format!("{}/{}/{}/{}", host_url, lang, prod_key, version)
}

//it's used to build API url
fn configs_to_url(api_confs: &ApiConfigs, resource_path: &str)
    -> Result<hyper::Uri, hyper::error::UriError> {
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

    url_str.parse::<Uri>()
}

fn build_ssl_client(wpool: &mut reactor::Core) -> hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
    let handle = wpool.handle();
    let ssl_connector = HttpsConnector::new(4, &handle).expect("Failed to init SSL connector");

    let client = Client::configure()
                    .connector(ssl_connector) //4threads?
                    .build( &handle );

    client
}

fn request_json<'a>(
    uri: &Uri,  proxy_confs: &'a ProxyConfigs, wpool: &mut reactor::Core
) -> Option<String> {
    let client = build_ssl_client( wpool );

    let req = client.get(uri.clone()).and_then(|res| { res.body().concat2() });

    let res = wpool.run(req).expect("Failed to run request");
    if let Some(content) = str::from_utf8(&res).ok(){
        Some(content.to_string())
    } else {
        None
    }

}

pub fn fetch_product_details_by_sha(confs: &Configs, file_sha: &str)
    -> Result<product::ProductMatch, Error> {

    let sha_res = fetch_product_by_sha(&confs, file_sha);
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

pub fn fetch_product_by_sha<'z>(confs: &Configs, sha: &str)
    -> Result<product::ProductMatch, io::Error> {
    let api_confs = confs.api.clone();
    let resource_path = format!(
        "products/sha/{}?api_key={}",
        encode_sha(sha), api_confs.key.clone().unwrap().as_str()
    );
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

    let mut wpool = reactor::Core::new()?;
    let json_txt = request_json( &resource_url, &confs.proxy, &mut wpool );
    process_sha_response(json_txt)
}

//replaces base64 special characters with HTML safe percentage encoding
//source: https://en.wikipedia.org/wiki/Base64#URL_applications
pub fn encode_sha<'a>(sha: &'a str) -> String {
    let encoded_sha = sha.to_string();

    encoded_sha.replace("+", "%2B")
        .replace("/", "%2F")
        .replace("=", "%3D")
        .trim().to_string()
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

pub fn fetch_product<'z>(
    confs: &Configs, lang: &str, prod_key: &str, version: &str
) -> Result<product::ProductMatch, io::Error> {

    let api_confs = confs.api.clone();
    let encoded_prod_key = encode_prod_key(&prod_key);
    let encoded_lang = encode_language(lang);
    let resource_path = format!(
        "products/{}/{}?prod_version={}&api_key={}",
        encoded_lang.clone(), encoded_prod_key.clone(), version, api_confs.key.clone().unwrap().as_str()
    );

    let prod_url = to_product_url(
        &confs.api,
        encoded_lang.clone().as_str(),
        prod_key,
        version
    );

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

    let mut wpool = reactor::Core::new()?;
    let json_txt = request_json( &resource_url, &confs.proxy, &mut wpool );
    process_product_response(json_txt, Some(prod_url))
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiError {
    error: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ShaItem {
    language: String,
    prod_key: String,
    version: String,
    sha_value: String,
    sha_method: String,
    prod_type: Option<String>,
    group_id: Option<String>,
    artifact_id: Option<String>,
    classifier: Option<String>,
    packaging: Option<String>
}

//-- helper functions
pub fn process_sha_response<'a>(json_text: Option<String> ) -> Result<product::ProductMatch, io::Error> {
    if json_text.is_none() {
        return Err(
            Error::new(ErrorKind::Other, "No response from API")
        )
    }

    let res: serde_json::Value = serde_json::from_str(&json_text.unwrap())?;

    if res.is_object() && res.get("error").is_some() {
        let e = Error::new(
            ErrorKind::Other,
            r#"API rate limit reached. Go to https://www.versioneye.com and upgrade your
                subscription to a higher plan."#
        );

        return Err(e);
    }

    if !res.is_array() {
        let e = Error::new( ErrorKind::Other, "Unsupported SHA response - expected array");
        return Err(e);
    }

    let shas = res.as_array().unwrap();
    if shas.len() == 0 {
        let e = Error::new( ErrorKind::Other, "No match for the SHA");
        return Err(e);
    }

    let doc:ShaItem = serde_json::from_value(shas[0].clone()).unwrap();
    let the_prod = product::Product {
        name: "".to_string(),
        language: doc.language,
        prod_key: doc.prod_key,
        version: doc.version,
        prod_type: doc.prod_type
    };

    let the_sha = product::ProductSHA {
        packaging: doc.packaging.unwrap_or("unknown".to_string()),
        method: doc.sha_method,
        value: doc.sha_value,
        filepath: None
    };

    Ok(product::ProductMatch::new(the_prod, the_sha))

}

// converts the response of product endpoint into ProductMatch struct
#[derive(Serialize, Deserialize, Debug)]
struct ProductItem {
    name: String,
    language: String,
    prod_key: String,
    version: String,
    prod_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LicenseItem {
    name: String,
    url: Option<String>
}

pub fn process_product_response<'a>(
    json_text: Option<String>, prod_url: Option<String>
) -> Result<product::ProductMatch, io::Error> {

    if json_text.is_none() {
        return Err(
            Error::new( ErrorKind::Other, "No response from API")
        )
    }

    let res: serde_json::Value = serde_json::from_str( &json_text.unwrap() )?;
    if !res.is_object() {
        return Err(Error::new(ErrorKind::Other, "No product details"));
    }

    //if response includes error field in HTTP200 response
    // NB! it may include other errors than limit, but @Rob asked to see custom Limit error message
    if res.is_object() && res.get("error").is_some() {
        let e = Error::new(
            ErrorKind::Other,
            r#"API rate limit reached. Go to https://www.versioneye.com and upgrade your
                subscription to a higher plan."#
        );

        return Err(e);
    }

    let product_doc:ProductItem = serde_json::from_value(res.clone())?;
    let the_prod = product::Product {
        name: product_doc.name,
        language: product_doc.language,
        prod_key: product_doc.prod_key,
        version: product_doc.version,
        prod_type: Some( product_doc.prod_type )
    };

    //extract license details
    let licenses = match res["licenses"].as_array() {
        Some(arr) => arr.iter().fold(vec![], |mut acc, ref x| {
            let lic_doc = x.as_object().unwrap();
            acc.push(product::ProductLicense {
                name: lic_doc["name"].as_str().unwrap_or("unknown").to_string(),
                url: lic_doc["url"].as_str().unwrap_or("").to_string()
            });

            acc
        }),
        None      => vec![]
    };

    //count number of vulnerabilities
    let n_vulns = match res["security_vulnerabilities"].as_array() {
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

