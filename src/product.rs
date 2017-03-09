extern crate rustc_serialize;
extern crate csv;

// Automatically generate `RustcDecodable` and `RustcEncodable` trait implementations
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Product {
    pub language: String,
    pub prod_key: String,
    pub version: String,
    pub name: String,
    pub prod_type: Option<String>
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ProductSHA {
    pub packaging: String,
    pub method: String,
    pub value: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ProductLicense {
    pub name: String,
    pub url: String
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct ProductMatch {
    pub sha: Option<ProductSHA>,
    pub product: Option<Product>,
    pub url: Option<String>,
    pub licenses: Vec<ProductLicense>,
    pub n_vulns: u32,
    pub filepath: Option<String>
}



impl ProductMatch {

    pub fn new(product: Product, sha: ProductSHA) -> ProductMatch {
        let url = format!(
            "https://www.versioneye.com/{}/{}",
            product.language.clone(),
            product.prod_key.clone()
        );

        ProductMatch {
            sha: Some(sha),
            product: Some(product),
            url: Some(url),
            licenses: vec![],
            n_vulns: 0,
            filepath: None
        }
    }

    pub fn empty() -> ProductMatch {
        ProductMatch {
            sha: None,
            product: None,
            url: None,
            licenses: vec![],
            n_vulns: 0,
            filepath: None
        }
    }
}


pub trait RowSerializer {
    fn to_fields(&self) -> Vec<String>;
    fn to_rows(&self) -> Vec<Vec<String>>;
}

//TODO: add header function
impl RowSerializer for ProductMatch {

    fn to_fields(&self) -> Vec<String> {
        vec![
            "filepath".to_string(), "packaging".to_string(), "sha_method".to_string(),
            "sha_value".to_string(), "language".to_string(), "prod_key".to_string(),
            "version".to_string(), "n_vulns".to_string(), "product_url".to_string(),
            "license".to_string()
        ]
    }

    fn to_rows(&self) -> Vec<Vec<String>> {
        let mut csv_row: Vec<String> = vec![];

        csv_row.push(self.filepath.clone().unwrap_or("".to_string()));

        csv_row = match self.sha.clone() {
            Some(x) => {
                csv_row.push(x.packaging);
                csv_row.push(x.method);
                csv_row.push(x.value);
                csv_row
            },
            None   => {
                let mut emp_row = vec!["".to_string(), "".to_string(), "".to_string()];
                csv_row.append(&mut emp_row);
                csv_row
            }
        };

        csv_row = match self.product.clone() {
            Some(x) => {
                csv_row.push(x.language);
                csv_row.push(x.prod_key);
                csv_row.push(x.version);

                csv_row
            },
            None    => {
                let mut emp_row = vec!["".to_string(), "".to_string(), "".to_string()];
                csv_row.append(&mut emp_row);
                csv_row
            }
        };

        csv_row.push( self.n_vulns.clone().to_string() );
        csv_row.push( self.url.clone().unwrap_or("".to_string()) );

        let mut rows = vec![];
        for lic in &self.licenses {
            let mut row = csv_row.to_vec();
            row.push(lic.name.clone().to_string());
            rows.push(row)
        }

        rows
    }

}