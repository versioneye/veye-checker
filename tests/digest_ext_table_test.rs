extern crate veye_checker;

use veye_checker::digest_ext_table::{DigestExtTable, DigestAlgo};

#[test]
fn test_digest_ext_table_init_with_default_values(){
    let ext_tbl = DigestExtTable::default();

    assert!(ext_tbl.is_md5("whl".to_string()));
    assert!(ext_tbl.is_sha1("jar".to_string()));
    assert!(ext_tbl.is_sha512("nupkg".to_string()));
}

#[test]
fn test_digest_ext_table_adding_new_extension_into_md5(){
    let mut ext_tbl = DigestExtTable::default();
    let file_ext = "tjar".to_string();

    assert!(ext_tbl.add(DigestAlgo::Md5, file_ext.clone()));
    assert!(ext_tbl.is_md5(file_ext))
}

#[test]
fn test_digest_ext_table_adding_new_extension_into_sha1(){
    let mut ext_tbl = DigestExtTable::default();
    let file_ext = "twar".to_string();

    assert!(ext_tbl.add(DigestAlgo::Sha1, file_ext.clone()));
    assert!(ext_tbl.is_sha1(file_ext))
}

#[test]
fn test_digest_ext_table_adding_new_extension_into_sha512(){
    let mut ext_tbl = DigestExtTable::default();
    let file_ext = "tnupkg".to_string();

    assert!(ext_tbl.add(DigestAlgo::Sha512, file_ext.clone()));
    assert!(ext_tbl.is_sha512(file_ext))
}

#[test]
fn test_digest_ext_table_swipes_default_table(){
    let mut ext_tbl = DigestExtTable::default();
    assert!(ext_tbl.swipe())
}