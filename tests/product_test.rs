extern crate veye_checker;

use veye_checker::product;
use veye_checker::product::RowSerializer;

#[test]
fn test_creating_product_sha(){
    let prod_sha = product::ProductSHA::from_sha("abc-123".to_string());
    assert_eq!("".to_string(), prod_sha.packaging);
    assert_eq!("".to_string(), prod_sha.method);
    assert_eq!("abc-123".to_string(), prod_sha.value);
    assert_eq!(true, prod_sha.filepath.is_none());
}

#[test]
fn test_creating_new_product_match(){
    let product = product::Product {
        language: "rust".to_string(),
        prod_key: "serde".to_string(),
        version: "1.0".to_string(),
        name: "serde".to_string(),
        prod_type: Some("cargo".to_string())
    };

    let prod_sha = product::ProductSHA::from_sha("abc-124".to_string());
    let prod_match = product::ProductMatch::new(product, prod_sha);
    let prod_url = "https://www.versioneye.com/rust/serde".to_string();

    assert_eq!(prod_url, prod_match.url.unwrap());
    assert_eq!(0, prod_match.licenses.len());
    assert_eq!(0, prod_match.n_vulns);
    assert_eq!(true, prod_match.error.is_none());

    let sha = prod_match.sha.expect("Sha value wasnt initialized for ProductMatch");
    assert_eq!("abc-124".to_string(), sha.value);
    assert_eq!("".to_string(), sha.packaging);
    assert_eq!(true, sha.filepath.is_none());

    let prod = prod_match.product.expect("product document wasnt initialized correctly");
    assert_eq!("rust".to_string(), prod.language);
    assert_eq!("serde".to_string(), prod.prod_key);
    assert_eq!("1.0".to_string(), prod.version);
    assert_eq!("serde".to_string(), prod.name);
    assert_eq!("cargo".to_string(), prod.prod_type.unwrap());
}

//it should returns correct order of field headers
#[test]
fn test_product_sha_to_fields(){
    let prod_sha = product::ProductSHA::from_sha("abc-125".to_string());
    let fields = prod_sha.to_fields();

    //it didnt change prod_sha itself
    assert_eq!("".to_string(), prod_sha.packaging);
    assert_eq!("".to_string(), prod_sha.method);
    assert_eq!("abc-125".to_string(), prod_sha.value);
    assert_eq!(true, prod_sha.filepath.is_none());

    //it returns correct headers
    assert_eq!(4, fields.len());
    assert_eq!("filepath".to_string(), fields[0]);
    assert_eq!("packaging".to_string(), fields[1]);
    assert_eq!("sha_method".to_string(), fields[2]);
    assert_eq!("sha_value".to_string(), fields[3]);

}

//it should returns correct list of list of values
#[test]
fn test_product_sha_to_rows(){
    let prod_sha = product::ProductSHA::from_sha("abc-126".to_string());
    let mut rows = prod_sha.to_rows();
    assert_eq!(1, rows.len());

    let row = rows.pop().expect("Failed to fetch product sha row");
    assert_eq!(4, row.len());
    assert_eq!("".to_string(), row[0]);
    assert_eq!("".to_string(), row[1]);
    assert_eq!("".to_string(), row[2]);
    assert_eq!("abc-126".to_string(), row[3]);
}


//it should returns correct list of productMatch fields
#[test]
fn test_product_match_to_fields(){
    let prod_match = product::ProductMatch::empty();
    let fields = prod_match.to_fields();

    //test values stay unaffected
    assert_eq!(true, prod_match.sha.is_none());
    assert_eq!(0, prod_match.licenses.len());
    assert_eq!(0, prod_match.n_vulns);

    //test correct order and values of fieldnames
    assert_eq!(11, fields.len());
    assert_eq!("filepath".to_string(), fields[0]);
    assert_eq!("packaging".to_string(), fields[1]);
    assert_eq!("sha_method".to_string(), fields[2]);
    assert_eq!("sha_value".to_string(), fields[3]);
    assert_eq!("language".to_string(), fields[4]);
    assert_eq!("prod_key".to_string(), fields[5]);
    assert_eq!("version".to_string(), fields[6]);
    assert_eq!("n_vulns".to_string(), fields[7]);
    assert_eq!("product_url".to_string(), fields[8]);
    assert_eq!("license".to_string(), fields[9]);
    assert_eq!("error".to_string(), fields[10]);
}

//it should return correct list of productMatch values
#[test]
fn test_product_match_to_rows(){
    let product = product::Product {
        language: "rust".to_string(),
        prod_key: "serde".to_string(),
        version: "1.0".to_string(),
        name: "serde".to_string(),
        prod_type: Some("cargo".to_string())
    };

    let prod_sha = product::ProductSHA::from_sha("abc-124".to_string());
    let prod_match = product::ProductMatch::new(product, prod_sha);
    let mut rows = prod_match.to_rows();

    //test that values stay unaffected
    assert_eq!(0, prod_match.n_vulns);
    assert_eq!(0, prod_match.licenses.len());

    let sha = prod_match.sha.expect("Sha value wasnt initialized for ProductMatch");
    assert_eq!("abc-124".to_string(), sha.value);
    assert_eq!("".to_string(), sha.packaging);
    assert_eq!(true, sha.filepath.is_none());

    //test does it returns correct row of values
    assert_eq!(1, rows.len());
    let row = rows.pop().expect("It should return= first row with values");
    let url = "https://www.versioneye.com/rust/serde".to_string();

    assert_eq!("".to_string(), row[0]);
    assert_eq!("".to_string(), row[1]);
    assert_eq!("".to_string(), row[2]);
    assert_eq!("abc-124".to_string(), row[3]);
    assert_eq!("rust".to_string(), row[4]);
    assert_eq!("serde".to_string(), row[5]);
    assert_eq!("1.0".to_string(), row[6]);
    assert_eq!("0".to_string(), row[7]);
    assert_eq!(url, row[8]);
    assert_eq!("unknown".to_string(), row[9]);
    assert_eq!("".to_string(), row[10]);
}

//TODO: test that to_rows() puts each license on the different line
#[test]
fn test_product_match_to_rows_licenses_separated_rows(){
    let mut prod_match = product::ProductMatch::empty();
    let lic1 = product::ProductLicense {
        name: "MIT".to_string(),
        url: "http://mit.edu".to_string()
    };
    let lic2 = product::ProductLicense {
        name: "EPL-1.0".to_string(),
        url: "http://epl.org".to_string()
    };

    prod_match.licenses.push(lic1);
    prod_match.licenses.push(lic2);

    let rows = prod_match.to_rows();
    //test that values stay unaffected
    assert_eq!(0, prod_match.n_vulns);
    assert_eq!(2, prod_match.licenses.len());

    //test does it return 2rows with correct license value
    assert_eq!(2, rows.len());
    assert_eq!("MIT".to_string(), rows[0][9]);
    assert_eq!("EPL-1.0".to_string(), rows[1][9]);
}