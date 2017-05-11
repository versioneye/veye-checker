extern crate veye_checker;

use std::path::PathBuf;
use std::fs::{self, File};
use std::io::Read;
use veye_checker::{tasks, product, configs, digest_ext_table};

#[test]
fn test_task_start_path_scanner(){
    let test_dir = PathBuf::from("test/fixtures/files");
    let ext_table = digest_ext_table::DigestExtTable::default();
    let (sha_ch, h1) = tasks::start_path_scanner(ext_table, test_dir);

    for sha in sha_ch.into_iter() {
        assert_eq!(true, sha.value.len() > 0);
        assert_eq!(true, sha.filepath.is_some());
    }

    h1.join().unwrap();

}

#[test]
fn test_task_start_path_scanner_folder_dont_exist(){
    let test_dir = PathBuf::from("test/fixtures/dont_exists");
    assert_eq!(false, test_dir.exists());

    let ext_table = digest_ext_table::DigestExtTable::default();
    let (_, h1) = tasks::start_path_scanner(ext_table, test_dir);

    let res = h1.join().unwrap();
    assert_eq!(true, res.is_err())

}

#[test]
fn test_task_start_sha_publisher(){
    let test_shas = vec![product::ProductSHA::from_sha("abc-123".to_string())];

    let (sha_ch, h1) = tasks::start_sha_publisher(test_shas);

    for sha in sha_ch.into_iter(){
        assert_eq!("abc-123".to_string(), sha.value);
    }

    let res = h1.join().unwrap();
    assert_eq!(true, res.is_ok());
}

#[test]
fn test_task_start_sha_publisher_with_empty_array(){
    let test_shas = vec![];
    let (_ , h1) = tasks::start_sha_publisher(test_shas);

    let res = h1.join().unwrap();
    assert_eq!(true, res.is_ok());
}

#[test]
#[cfg(feature="api")]
fn test_api_task_start_sha_fetcher(){
    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let confs = configs::read_configs(None); //dont forget to specify API_KEY
    let test_shas = vec![
        product::ProductSHA::from_sha(file_sha.to_string())
    ];

    let (sha_ch, h1) = tasks::start_sha_publisher(test_shas);
    let (prod_ch, h2) = tasks::start_sha_fetcher(confs, sha_ch);

    for res in prod_ch.into_iter() {
        assert_eq!(true, res.sha.is_some());

        let sha = res.sha.unwrap();
        assert_eq!("".to_string(), sha.packaging); //it keeps original sha doc
        assert_eq!("".to_string(), sha.method);
        assert_eq!(file_sha.to_string(), sha.value);
        assert_eq!(None, sha.filepath);

        assert_eq!(true, res.product.is_some());
        let prod = res.product.unwrap();
        assert_eq!("java".to_string(), prod.language);
        assert_eq!("Maven2".to_string(), prod.prod_type.unwrap());
        assert_eq!("commons-beanutils/commons-beanutils".to_string(), prod.prod_key);
        assert_eq!("1.7.0".to_string(), prod.version);
        assert_eq!("commons-beanutils".to_string(), prod.name);

    }

    let res1 = h1.join().unwrap();
    assert_eq!(true, res1.is_ok());
    let res2 = h2.join().unwrap();
    assert_eq!(true, res2.is_ok());
}

#[test]
#[cfg(feature="api")]
fn test_api_task_start_sha_fetcher_sha_dont_exists(){
    let file_sha = "abc-123-dont-exists";
    let confs = configs::read_configs(None);
    let test_shas = vec![
        product::ProductSHA::from_sha(file_sha.to_string())
    ];

    let (sha_ch, h1) = tasks::start_sha_publisher(test_shas);
    let (prod_ch, h2) = tasks::start_sha_fetcher(confs, sha_ch);

    //it should return ProductMatch with original sha and empty prod info
    for res in prod_ch.into_iter() {
        assert_eq!(true, res.sha.is_some());
        let sha = res.sha.unwrap();
        assert_eq!("".to_string(), sha.packaging); //it keeps original sha doc
        assert_eq!("".to_string(), sha.method);
        assert_eq!(file_sha.to_string(), sha.value);
        assert_eq!(None, sha.filepath);

        assert_eq!(true, res.product.is_none());
    }

    let res1 = h1.join().unwrap();
    assert_eq!(true, res1.is_ok());
    let res2 = h2.join().unwrap();
    assert_eq!(true, res2.is_ok());
}

#[test]
fn test_task_start_sha_csv_writer(){
    let confs = configs::read_configs(None);
    let file_sha = "5675fd96b29656504b86029551973d60fb41339b";
    let test_shas = vec![
        product::ProductSHA::from_sha(file_sha.to_string())
    ];
    let outpath = PathBuf::from("temp/test_task_sha_writer.csv");
    let expected_content = "filepath;packaging;sha_method;sha_value\n;;;5675fd96b29656504b86029551973d60fb41339b\n";

    let (sha_ch, h1) = tasks::start_sha_publisher(test_shas);
    let h2 = tasks::start_sha_csv_writer(outpath.clone(), confs.csv, sha_ch);

    let res1 = h1.join().unwrap();
    assert_eq!(true, res1.is_ok());
    let res2 = h2.join().unwrap();
    assert_eq!(true, res2.is_ok());

    let f_res = File::open(outpath.clone().as_path());
    assert_eq!(true, f_res.is_ok());
    let mut fd = f_res.unwrap();
    let mut content = String::new();
    fd.read_to_string(&mut content).unwrap();
    assert_eq!(expected_content.to_string(), content);

    fs::remove_file(outpath.as_path()).expect("Failed to delete test_task_start_sha file");
}

#[test]
fn test_task_start_sha_csv_writer_empty_input(){
    let confs = configs::read_configs(None);
    let test_shas = vec![];
    let outpath = PathBuf::from("temp/test_task_sha_writer_empty.csv");
    let expected_content = "";

    let (sha_ch, h1) = tasks::start_sha_publisher(test_shas);
    let h2 = tasks::start_sha_csv_writer(outpath.clone(), confs.csv, sha_ch);

    let res1 = h1.join().unwrap();
    assert_eq!(true, res1.is_ok());
    let res2 = h2.join().unwrap();
    assert_eq!(true, res2.is_ok());

    let f_res = File::open(outpath.clone().as_path());
    assert_eq!(true, f_res.is_ok());
    let mut fd = f_res.unwrap();
    let mut content = String::new();
    fd.read_to_string(&mut content).unwrap();
    assert_eq!(expected_content.to_string(), content);

    fs::remove_file(outpath.as_path()).expect("Failed to delete test_task_start_sha file");
}

#[test]
fn test_task_start_product_csv_writer(){
    let outpath = PathBuf::from("temp/test_task_product_writer.csv");
    let confs = configs::read_configs(None);

    let test_prods = vec![ product::ProductMatch::empty() ];
    let expected_content = "filepath;packaging;sha_method;sha_value;language;prod_key;version;n_vulns;product_url;license;error\n;;;;;;;0;;unknown;\n";

    let (prod_ch, h1) = tasks::start_product_match_publisher(test_prods);
    let h2 = tasks::start_product_csv_writer(outpath.clone(), confs.csv, prod_ch);

    let res1 = h1.join().unwrap();
    assert_eq!(true, res1.is_ok());
    let res2 = h2.join().unwrap();
    assert_eq!(true, res2.is_ok());

    let f_res = File::open(outpath.clone().as_path());
    assert_eq!(true, f_res.is_ok());
    let mut fd = f_res.unwrap();
    let mut content = String::new();
    fd.read_to_string(&mut content).unwrap();
    assert_eq!(expected_content.to_string(), content);

    fs::remove_file(outpath.as_path()).expect("Failed to delete test_task_start_sha file");
}

#[test]
fn test_task_start_product_csv_writer_empty_rows(){
    let outpath = PathBuf::from("temp/test_task_product_writer_empty.csv");
    let test_prods = vec![];
    let expected_content = "";
    let confs = configs::read_configs(None);

    let (prod_ch, h1) = tasks::start_product_match_publisher(test_prods);
    let h2 = tasks::start_product_csv_writer(outpath.clone(), confs.csv, prod_ch);

    let res1 = h1.join().unwrap();
    assert_eq!(true, res1.is_ok());
    let res2 = h2.join().unwrap();
    assert_eq!(true, res2.is_ok());

    let f_res = File::open(outpath.clone().as_path());
    assert_eq!(true, f_res.is_ok());
    let mut fd = f_res.unwrap();
    let mut content = String::new();
    fd.read_to_string(&mut content).unwrap();
    assert_eq!(expected_content.to_string(), content);

    fs::remove_file(outpath.as_path()).expect("Failed to delete test_task_start_sha file");
}