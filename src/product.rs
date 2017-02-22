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

}


pub trait CSVSerializer {
    fn to_csv_header(&self) -> String;
    fn to_csv(&self) -> String;
}


//TODO: add header function

impl CSVSerializer for ProductMatch {
    fn to_csv_header(&self) -> String {
        let headers = (
            "filepath", "packaging", "sha_method", "sha_value", "language",
            "prod_key", "version", "n_vulns", "product_url", "license"
        );

        let mut wtr = csv::Writer::from_memory();
        wtr.encode(headers);
        wtr.as_string().to_string()
    }

    fn to_csv(&self) -> String {

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


        let mut wtr = csv::Writer::from_memory();
        for lic in &self.licenses {
            let mut row = csv_row.to_vec();
            row.push(lic.name.clone().to_string());
            wtr.encode(row);
        }

        wtr.as_string().to_string()
    }

}