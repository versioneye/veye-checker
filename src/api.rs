use std::io::{self, Error, ErrorKind};

use hyper;

use hyper_tls::HttpsConnector;
use url::Url;

use configs::{ApiConfigs, Configs, ProxyConfigs};
use futures::executor::block_on;
use hyper::body::to_bytes;
use product;
use serde_json;

const HOST_URL: &'static str = "https://www.versioneye.com";

//it is used to build url to the product page (SAAS or Enterprise)
pub fn to_product_url(api_confs: &ApiConfigs, lang: &str, prod_key: &str, version: &str) -> String {
    let scheme = match api_confs.scheme.clone() {
        Some(val) => val,
        None => "http".to_string(),
    };
    let host = match api_confs.host.clone() {
        Some(val) => val,
        None => HOST_URL.to_string(),
    };

    let host_url = match api_confs.port.clone() {
        Some(port) => format!("{}://{}:{}", scheme, host, port),
        None => format!("{}://{}", scheme, host),
    };

    format!("{}/{}/{}/{}", host_url, lang, prod_key, version)
}

//it's used to build API url
fn configs_to_url(api_confs: &ApiConfigs, resource_path: &str) -> Result<Url, url::ParseError> {
    let url_str = match api_confs.port {
        None => format!(
            "{}://{}/{}/{}",
            api_confs.scheme.clone().unwrap(),
            api_confs.host.clone().unwrap(),
            api_confs.path.clone().unwrap(),
            resource_path,
        ),
        Some(port) => format!(
            "{}://{}:{}/{}/{}",
            api_confs.scheme.clone().unwrap(),
            api_confs.host.clone().unwrap(),
            port,
            api_confs.path.clone().unwrap(),
            resource_path
        ),
    };

    Url::parse(url_str.as_str())
}

enum Client {
    Proxy(
        hyper::Client<
            hyper_proxy::ProxyConnector<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
        >,
    ),
    NoProxy(hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>),
}

impl Client {
    pub fn get(&self, uri: http::Uri) -> hyper::client::ResponseFuture {
        match self {
            Client::Proxy(client) => client.get(uri),
            Client::NoProxy(client) => client.get(uri),
        }
    }
}

fn request_json<'a>(uri: &Url, proxy_confs: &'a ProxyConfigs) -> Option<String> {
    let connector = HttpsConnector::new();
    let proxy_confs = proxy_confs.clone();

    let client = if proxy_confs.is_complete() {
        let uri = if let Some(port) = proxy_confs.port {
            format!(
                "{}://{}:{}",
                proxy_confs.scheme.unwrap(),
                proxy_confs.host.unwrap(),
                port
            )
        } else {
            format!(
                "{}://{}",
                proxy_confs.scheme.unwrap(),
                proxy_confs.host.unwrap()
            )
        };

        let uri = uri.parse().unwrap();

        let proxy = hyper_proxy::Proxy::new(hyper_proxy::Intercept::All, uri);

        let proxy_connector = hyper_proxy::ProxyConnector::from_proxy(connector, proxy).unwrap();

        Client::Proxy(hyper::Client::builder().build(proxy_connector))
    } else {
        Client::NoProxy(hyper::Client::builder().build(connector))
    };

    let uri = uri.as_str().parse().unwrap();

    let res = block_on(client.get(uri)).expect("Failed to fetch results from the url");

    let response_bytes = block_on(to_bytes(res.into_body())).expect("Failed to read response body");

    let response_string =
        String::from_utf8(response_bytes.to_vec()).expect("Failed to read response body");

    Some(response_string)
}

pub fn fetch_product_details_by_sha(
    confs: &Configs,
    file_sha: &str,
) -> Result<product::ProductMatch, Error> {
    let sha_res = fetch_product_by_sha(&confs, file_sha);
    match sha_res {
        Ok(m) => {
            let sha = m.sha.expect("No product sha from SHA result");
            let product = m.product.expect("No product info from SHA result");
            match fetch_product(
                &confs,
                &product.language,
                &product.prod_key,
                &product.version,
            ) {
                Ok(mut m) => {
                    m.sha = Some(sha);
                    Ok(m)
                }
                Err(e) => {
                    println!("Failed to fetch product details for sha: {}", file_sha);
                    Err(e)
                }
            }
        }
        Err(e) => Err(e),
    }
}

pub fn fetch_product_by_sha(
    confs: &Configs,
    sha: &str,
) -> Result<product::ProductMatch, io::Error> {
    let api_confs = confs.api.clone();
    let resource_path = format!("products/sha/{}", encode_sha(sha));
    let mut resource_url = match configs_to_url(&api_confs, resource_path.as_str()) {
        Ok(the_url) => the_url,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "The values of API configs make up non-valid URL",
            ))
        }
    };

    //attach query params
    resource_url
        .query_pairs_mut()
        .clear()
        .append_pair("api_key", api_confs.key.clone().unwrap().as_str());

    let json_txt = request_json(&resource_url, &confs.proxy);
    process_sha_response(json_txt)
}

//replaces base64 special characters with HTML safe percentage encoding
//source: https://en.wikipedia.org/wiki/Base64#URL_applications
pub fn encode_sha<'a>(sha: &'a str) -> String {
    let encoded_sha = sha.to_string();

    encoded_sha
        .replace("+", "%2B")
        .replace("/", "%2F")
        .replace("=", "%3D")
        .trim()
        .to_string()
}

pub fn encode_prod_key<'b>(prod_key: &'b str) -> String {
    let encoded_prod_key = prod_key.to_string();
    encoded_prod_key
        .replace(".", "~")
        .replace("/", ":")
        .trim()
        .to_string()
}

pub fn encode_language<'b>(lang: &'b str) -> String {
    let encoded_lang = lang.to_string();
    encoded_lang
        .replace(".", "")
        .trim()
        .to_lowercase()
        .to_string()
}

pub fn fetch_product<'a>(
    confs: &Configs,
    lang: &str,
    prod_key: &str,
    version: &str,
) -> Result<product::ProductMatch, io::Error> {
    let api_confs = confs.api.clone();
    let encoded_prod_key = encode_prod_key(&prod_key);
    let encoded_lang = encode_language(lang);
    let resource_path = format!(
        "products/{}/{}",
        encoded_lang.clone(),
        encoded_prod_key.clone()
    );
    let prod_url = to_product_url(&confs.api, encoded_lang.clone().as_str(), prod_key, version);

    let mut resource_url = match configs_to_url(&api_confs, resource_path.as_str()) {
        Ok(the_url) => the_url,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "The values of API configs make up non-valid URL",
            ))
        }
    };

    //attach query params
    resource_url
        .query_pairs_mut()
        .clear()
        .append_pair("prod_version", version)
        .append_pair("api_key", api_confs.key.clone().unwrap().as_str());

    let json_txt = request_json(&resource_url, &confs.proxy);
    process_product_response(json_txt, Some(prod_url))
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiError {
    error: String,
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
    packaging: Option<String>,
}

//-- helper functions
pub fn process_sha_response(json_text: Option<String>) -> Result<product::ProductMatch, io::Error> {
    if json_text.is_none() {
        return Err(Error::new(ErrorKind::Other, "No response from API"));
    }

    let res: serde_json::Value = serde_json::from_str(json_text.unwrap().as_str())?;

    if res.is_object() && res.get("error").is_some() {
        let e = Error::new(
            ErrorKind::Other,
            r#"API rate limit reached. Go to https://www.versioneye.com and upgrade your
                subscription to a higher plan."#,
        );

        return Err(e);
    }

    if !res.is_array() {
        let e = Error::new(
            ErrorKind::Other,
            "Unsupported SHA response - expected array",
        );
        return Err(e);
    }

    let shas = res.as_array().unwrap();
    if shas.len() == 0 {
        let e = Error::new(ErrorKind::Other, "No match for the SHA");
        return Err(e);
    }

    let doc: ShaItem = serde_json::from_value(shas[0].clone()).unwrap();
    let the_prod = product::Product {
        name: "".to_string(),
        language: doc.language,
        prod_key: doc.prod_key,
        version: doc.version,
        prod_type: doc.prod_type,
    };

    let the_sha = product::ProductSHA {
        packaging: doc.packaging.unwrap_or("unknown".to_string()),
        method: doc.sha_method,
        value: doc.sha_value,
        filepath: None,
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
    url: Option<String>,
}

pub fn process_product_response(
    json_text: Option<String>,
    prod_url: Option<String>,
) -> Result<product::ProductMatch, io::Error> {
    if json_text.is_none() {
        return Err(Error::new(ErrorKind::Other, "No response from API"));
    }

    let res: serde_json::Value = serde_json::from_str(&json_text.unwrap().as_str())?;
    if !res.is_object() {
        return Err(Error::new(ErrorKind::Other, "No product details"));
    }

    //if response includes error field in HTTP200 response
    // NB! it may include other errors than limit, but @Rob asked to see custom Limit error message
    if res.is_object() && res.get("error").is_some() {
        let e = Error::new(
            ErrorKind::Other,
            r#"API rate limit reached. Go to https://www.versioneye.com and upgrade your
                subscription to a higher plan."#,
        );

        return Err(e);
    }

    let product_doc: ProductItem = serde_json::from_value(res.clone())?;
    let the_prod = product::Product {
        name: product_doc.name,
        language: product_doc.language,
        prod_key: product_doc.prod_key,
        version: product_doc.version,
        prod_type: Some(product_doc.prod_type),
    };

    //extract license details
    let licenses = match res["licenses"].as_array() {
        Some(arr) => arr.iter().fold(vec![], |mut acc, ref x| {
            let lic_doc = x.as_object().unwrap();
            acc.push(product::ProductLicense {
                name: lic_doc["name"].as_str().unwrap_or("unknown").to_string(),
                url: lic_doc["url"].as_str().unwrap_or("").to_string(),
            });

            acc
        }),
        None => vec![],
    };

    //count number of vulnerabilities
    let n_vulns = match res["security_vulnerabilities"].as_array() {
        Some(arr) => arr.len() as u32,
        None => 0 as u32,
    };

    let the_match = product::ProductMatch {
        sha: None,
        product: Some(the_prod),
        url: prod_url,
        licenses: licenses,
        n_vulns: n_vulns,
        error: None,
    };

    Ok(the_match)
}
