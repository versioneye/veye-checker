extern crate rustc_serialize;
extern crate csv;


pub type CSVStringRow = Vec<String>;

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
    pub value: String,
    pub filepath: Option<String>
}

impl ProductSHA {
    pub fn from_sha(sha_value: String) -> ProductSHA {
        ProductSHA {
            packaging: "".to_string(),
            method: "".to_string(),
            value: sha_value,
            filepath: None
        }
    }
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
    pub error: Option<String>
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
            error: None
        }
    }

    pub fn empty() -> ProductMatch {
        ProductMatch {
            sha: None,
            product: None,
            url: None,
            licenses: vec![],
            n_vulns: 0,
            error: None
        }
    }
}


pub trait RowSerializer {
    fn to_fields(&self) -> CSVStringRow;
    fn to_rows(&self) -> Vec<CSVStringRow>;
}

impl RowSerializer for ProductSHA {
    fn to_fields(&self) -> CSVStringRow {
      vec![
          "filepath".to_string(), "packaging".to_string(),
          "sha_method".to_string(), "sha_value".to_string()
      ]
    }

    fn to_rows(&self) -> Vec<CSVStringRow> {
        let filepath = match self.filepath.clone() {
            Some(path) => path,
            None => "".to_string()
        };

        let csv_row = vec![
            filepath, self.packaging.clone(),
            self.method.clone(), self.value.clone()
        ];

        vec![csv_row]
    }
}

impl RowSerializer for ProductMatch {

    fn to_fields(&self) -> CSVStringRow {
        vec![
            "filepath".to_string(), "packaging".to_string(), "sha_method".to_string(),
            "sha_value".to_string(), "language".to_string(), "prod_key".to_string(),
            "version".to_string(), "n_vulns".to_string(), "product_url".to_string(),
            "license".to_string(), "error".to_string()
        ]
    }

    fn to_rows(&self) -> Vec<CSVStringRow> {
        let mut csv_row: CSVStringRow = vec![];

        csv_row = match self.sha.clone() {
            Some(x) => {
                let mut sha_rows = x.to_rows().pop().unwrap();
                csv_row.append(&mut sha_rows);
                csv_row
            },
            None   => {
                let mut emp_row = vec![
                    "".to_string(), "".to_string(), "".to_string(), "".to_string()
                ];
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

        if self.licenses.len() > 0 {
            // split every license into own line
            for lic in &self.licenses {
                let mut row = csv_row.to_vec();
                row.push(lic.name.clone().to_string());
                row.push(self.error.clone().unwrap_or("".to_string()));
                rows.push(row);
            }
        } else {
            csv_row.push("unknown".to_string()); //when response had no license information
            csv_row.push(self.error.clone().unwrap_or("".to_string()));
            rows.push(csv_row);
        }

        rows
    }

}