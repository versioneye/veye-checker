extern crate veye_checker;

use std::path::PathBuf;
use veye_checker::{tasks, product};

#[test]
fn test_task_start_path_scanner(){
    let test_dir = PathBuf::from("test/fixtures/files");
    let (sha_ch, h1) = tasks::start_path_scanner(test_dir);

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

    let (sha_ch, h1) = tasks::start_path_scanner(test_dir);

    let res = h1.join().unwrap();
    assert_eq!(true, res.is_err())

}

#[test]
fn test_task_start_sha_publisher(){
    let test_shas = vec![product::ProductSHA::from_sha("abc-123".to_string())];

    let (sha_ch, h1) = tasks::start_sha_publisher(test_shas);

    for sha in sha_ch.into_iter(){
        println!("...");
        assert_eq!("abc-123".to_string(), sha.value);
    }

    let res = h1.join().unwrap();
    assert_eq!(true, res.is_ok());
}

#[test]
fn test_task_start_sha_publisher_with_empty_array(){
    let test_shas = vec![];
    let (sha_ch, h1) = tasks::start_sha_publisher(test_shas);

    let res = h1.join().unwrap();
    assert_eq!(true, res.is_ok());
}