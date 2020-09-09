extern crate veye_checker;

use std::path::Path;
use veye_checker::checker;
use veye_checker::digest_ext_table;

#[test]
fn test_checker_digest_sha1() {
    let jar_file_path = Path::new("tests/fixtures/files/test.jar");
    let correct_sha = "5675fd96b29656504b86029551973d60fb41339b".to_string();

    match checker::digest_sha1(&jar_file_path) {
        Ok(sha_val) => assert_eq!(correct_sha, sha_val),
        Err(e) => {
            println!("Failed to test digest_sha - {:?}", e);
            assert!(false);
        }
    }
}

#[test]
fn test_checker_digest_sha512() {
    let nupkg_file_path = Path::new("tests/fixtures/files/test.nupkg");
    let correct_sha =
        "U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw=="
            .to_string();

    match checker::digest_sha512b64(&nupkg_file_path) {
        Ok(sha_val) => assert_eq!(correct_sha, sha_val),
        Err(e) => {
            println!("Failed to test digest512b64 - {:?}", e);
            assert!(false)
        }
    }
}

#[test]
fn test_checker_digest_md5() {
    let file_path = Path::new("tests/fixtures/files/pypi.tar.gz");
    let correct_md5 = "fe7daf822f1d36d1bd37ac41cf5817e7".to_string();
    match checker::digest_md5(&file_path) {
        Ok(md5_val) => assert_eq!(correct_md5, md5_val),
        Err(e) => {
            println!("Failed to test digest_md5 - {:?}", e);
            assert!(false);
        }
    };
}

#[test]
fn test_checker_digest_file_with_jar() {
    let ext_table = digest_ext_table::DigestExtTable::default();
    let jar_file_path = Path::new("tests/fixtures/files/test.jar");
    let correct_sha = "5675fd96b29656504b86029551973d60fb41339b".to_string();

    match checker::digest_file(&ext_table, &jar_file_path) {
        Some(shas) => {
            assert_eq!(1, shas.len());
            assert_eq!("sha1".to_string(), shas[0].method);
            assert_eq!(correct_sha, shas[0].value);
        }
        None => {
            println!("Failed to test digest_file with Jar file.");
            assert!(false);
        }
    }
}

#[test]
fn test_checker_digest_file_with_pypi() {
    let ext_table = digest_ext_table::DigestExtTable::default();
    let pypi_file_path = Path::new("tests/fixtures/files/pypi.tar.gz");
    let correct_md5 = "fe7daf822f1d36d1bd37ac41cf5817e7".to_string();

    match checker::digest_file(&ext_table, &pypi_file_path) {
        Some(shas) => {
            assert_eq!(1, shas.len());
            assert_eq!("md5".to_string(), shas[0].method);
            assert_eq!(correct_md5, shas[0].value);
        }
        None => {
            println!("failed to test digest_file with Pypi file");
            assert!(false);
        }
    }
}

#[test]
fn test_checker_digest_file_with_nuget() {
    let ext_table = digest_ext_table::DigestExtTable::default();
    let nupkg_file_path = Path::new("tests/fixtures/files/test.nupkg");
    let correct_sha =
        "U82mHQSKaIk+lpSVCbWYKNavmNH1i5xrExDEquU1i6I5pV6UMOqRnJRSlKO3cMPfcpp0RgDY+8jUXHdQ4IfXvw=="
            .to_string();

    match checker::digest_file(&ext_table, &nupkg_file_path) {
        Some(shas) => {
            assert_eq!(1, shas.len());
            assert_eq!("sha512".to_string(), shas[0].method);
            assert_eq!(correct_sha, shas[0].value);
        }
        None => {
            println!("failed to test digest_file with Nuget file");
            assert!(false);
        }
    }
}

#[test]
fn test_checker_digest_file_block_algo() {
    let mut ext_table = digest_ext_table::DigestExtTable::default();
    let pypi_file_path = Path::new("tests/fixtures/files/pypi.tar.gz");
    let correct_md5 = "fe7daf822f1d36d1bd37ac41cf5817e7".to_string();

    ext_table.block(digest_ext_table::DigestAlgo::Md5);
    assert_eq!(false, ext_table.is_md5("gz".to_string()));

    match checker::digest_file(&ext_table, &pypi_file_path) {
        Some(shas) => {
            println!("failed to block using MD5 algo for Pypi files");
            assert!(false);
        }
        None => assert!(true),
    };
}

#[test]
fn test_checker_digest_file_change_algo() {
    let mut ext_table = digest_ext_table::DigestExtTable::default();
    let jar_file_path = Path::new("tests/fixtures/files/test.jar");
    let correct_md5 = "0f18acf5fa857f9959675e14d901a7ce".to_string();

    ext_table.swipe();
    ext_table.add(digest_ext_table::DigestAlgo::Md5, "jar".to_string());
    assert_eq!(true, ext_table.is_md5("jar".to_string()));

    match checker::digest_file(&ext_table, &jar_file_path) {
        Some(shas) => {
            assert_eq!(1, shas.len());
            assert_eq!("md5".to_string(), shas[0].method);
            assert_eq!(correct_md5, shas[0].value);
        }
        None => {
            println!("failed to test changing of algo for Jar file");
            assert!(false);
        }
    }
}

#[test]
fn test_checker_digest_file_multiple_algo() {
    let mut ext_table = digest_ext_table::DigestExtTable::default();
    let jar_file_path = Path::new("tests/fixtures/files/test.jar");
    let correct_sha = "5675fd96b29656504b86029551973d60fb41339b".to_string();
    let correct_md5 = "0f18acf5fa857f9959675e14d901a7ce".to_string();

    ext_table.add(digest_ext_table::DigestAlgo::Md5, "jar".to_string());
    assert_eq!(true, ext_table.is_md5("jar".to_string()));
    assert_eq!(true, ext_table.is_sha1("jar".to_string()));

    match checker::digest_file(&ext_table, &jar_file_path) {
        Some(shas) => {
            assert_eq!(2, shas.len());

            assert_eq!("md5".to_string(), shas[0].method);
            assert_eq!(correct_md5, shas[0].value);

            assert_eq!("sha1".to_string(), shas[1].method);
            assert_eq!(correct_sha, shas[1].value);
        }
        None => {
            println!("failed to test usage of multiple algos for Jar file");
            assert!(false);
        }
    }
}
