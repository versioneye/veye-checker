extern crate veye_checker;

use std::path::Path;
use veye_checker::checker;

#[test]
fn digest_jar_test(){
    let jar_file_path = Path::new("tests/fixtures/files/test.jar");
    let correct_sha = "5675fd96b29656504b86029551973d60fb41339b".to_string();

    let prod_sha = checker::digest_jar(&jar_file_path);

    assert_eq!("jar".to_string(), prod_sha.packaging);
    assert_eq!("sha1".to_string(), prod_sha.method);
    assert_eq!(correct_sha, prod_sha.value);
    assert_eq!(true, prod_sha.filepath.is_some())
}

#[test]
fn digest_nupkg_test(){
    let nupkg_file_path = Path::new("tests/fixtures/files/test.nupkg");
    let correct_sha = "U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==".to_string();

    let prod_sha = checker::digest_nupkg(&nupkg_file_path);

    assert_eq!("nupkg".to_string(), prod_sha.packaging);
    assert_eq!("sha512".to_string(), prod_sha.method);
    assert_eq!(correct_sha, prod_sha.value);
    assert_eq!(true, prod_sha.filepath.is_some());
}

//it should correctly dispatch the nupkg-digestor
#[test]
fn digest_file_nupkg_test(){
    let nupkg_file_path = Path::new("tests/fixtures/files/test.nupkg");
    let correct_sha = "U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==".to_string();

    if let Some(prod_sha) = checker::digest_file(nupkg_file_path) {
        assert_eq!("nupkg".to_string(), prod_sha.packaging);
        assert_eq!("sha512".to_string(), prod_sha.method);
        assert_eq!(correct_sha, prod_sha.value);
        assert_eq!(true, prod_sha.filepath.is_some());
    } else {
        assert_eq!("", "It didnt return any product sha value");
    }
}

//it should dispatch correctly the jar-digestor
#[test]
fn digest_file_jar_test(){
    let jar_file_path = Path::new("tests/fixtures/files/test.jar");
    let correct_sha = "5675fd96b29656504b86029551973d60fb41339b".to_string();

    if let Some(prod_sha) = checker::digest_file(&jar_file_path) {
        assert_eq!("jar".to_string(), prod_sha.packaging);
        assert_eq!("sha1".to_string(), prod_sha.method);
        assert_eq!(correct_sha, prod_sha.value);
        assert_eq!(true, prod_sha.filepath.is_some())
    } else {
        assert_eq!("", "Failed to dispatch jar file and returned None")
    }


}