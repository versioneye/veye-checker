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
    assert!(prod_sha.filepath.is_some())
}

#[test]
fn digest_nupkg_test(){
    let nupkg_file_path = Path::new("tests/fixtures/files/test.nupkg");
    let correct_sha = "U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw==".to_string();

    let prod_sha = checker::digest_nupkg(&nupkg_file_path);

    assert_eq!("nupkg".to_string(), prod_sha.packaging);
    assert_eq!("sha512".to_string(), prod_sha.method);
    assert_eq!(correct_sha, prod_sha.value);
    assert!(prod_sha.filepath.is_some());
}


#[test]
fn digest_pypi_test(){
    let file_path = Path::new("tests/fixtures/files/pypi.tar.gz");
    let correct_md5 = "fe7daf822f1d36d1bd37ac41cf5817e7".to_string();
    let prod_sha = checker::digest_pypi(&file_path);

    assert_eq!("pypi".to_string(), prod_sha.packaging);
    assert_eq!("md5".to_string(), prod_sha.method);
    assert_eq!(correct_md5, prod_sha.value);
    assert!(prod_sha.filepath.is_some());
}

#[test]
fn digest_pypi_whl_test(){
    let file_path = Path::new("tests/fixtures/files/pypi.whl");
    let correct_md5 = "ffa1ee60be515c04b4c13fd13feea27a".to_string();
    let prod_sha = checker::digest_pypi(&file_path);

    assert_eq!("pypi".to_string(), prod_sha.packaging);
    assert_eq!("md5".to_string(), prod_sha.method);
    assert_eq!(correct_md5, prod_sha.value);
    assert!(prod_sha.filepath.is_some());
}

#[test]
fn digest_npm_test(){
    let file_path = Path::new("tests/fixtures/files/npm.tgz");
    let correct_sha = "6f631aef336d6c46362b51764044ce216be3c051".to_string();
    let prod_sha = checker::digest_npm(&file_path);

    assert_eq!("npm".to_string(), prod_sha.packaging);
    assert_eq!("sha1".to_string(), prod_sha.method);
    assert_eq!(correct_sha, prod_sha.value);
    assert!(prod_sha.filepath.is_some());
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
        assert!(prod_sha.filepath.is_some());
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
        assert!(prod_sha.filepath.is_some());
    } else {
        assert_eq!("", "Failed to dispatch jar file and returned None");
    }

}

//it should dispatch correctly *.tar.gz files on pypi
#[test]
fn digest_file_pypi_tar_test(){
    let tar_file_path = Path::new("tests/fixtures/files/pypi.tar.gz");
    let correct_md5 = "fe7daf822f1d36d1bd37ac41cf5817e7".to_string();

    if let Some(prod_sha) = checker::digest_file(&tar_file_path) {
        assert_eq!("pypi".to_string(), prod_sha.packaging);
        assert_eq!("md5".to_string(), prod_sha.method);
        assert_eq!(correct_md5, prod_sha.value);
        assert!(prod_sha.filepath.is_some());
    } else {
        assert_eq!("", "It failed to return digest for PYPI tar file");
    }
}

// it should correctly dispatch PYPI wheel files
#[test]
fn digest_file_pypi_whl_test(){
    let whl_file_path = Path::new("tests/fixtures/files/pypi.whl");
    let correct_md5 = "ffa1ee60be515c04b4c13fd13feea27a".to_string();

    if let Some(prod_sha) = checker::digest_file(&whl_file_path) {
        assert_eq!("pypi".to_string(), prod_sha.packaging);
        assert_eq!("md5".to_string(), prod_sha.method);
        assert_eq!(correct_md5, prod_sha.value);
        assert!(prod_sha.filepath.is_some());
    } else {
        assert_eq!("", "It failed to return digest for PYPI wheel file");
    }
}

#[test]
fn digest_file_npm_test(){
    let file_path = Path::new("tests/fixtures/files/npm.tgz");
    let correct_sha = "6f631aef336d6c46362b51764044ce216be3c051".to_string();

    if let Some(prod_sha) = checker::digest_file(&file_path) {
        assert_eq!("npm".to_string(), prod_sha.packaging);
        assert_eq!("sha1".to_string(), prod_sha.method);
        assert_eq!(correct_sha, prod_sha.value);
        assert!(prod_sha.filepath.is_some());
    } else {
        assert_eq!("", "It failed to return digest for NPM tgz file");
    }
}