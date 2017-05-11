use sha1::{Sha1};
use sha2::{Sha512, Digest};
use base64;
use md5;

use std::io::{self, Error, ErrorKind};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use product::ProductSHA;
use digest_ext_table::{DigestAlgo, DigestExtTable};


pub fn digest_sha1(filepath: &Path) -> Result<String, Error> {
    let mut f = File::open(filepath).ok().expect("Failed to read file");
    let mut buffer = Vec::new();

    let mut hasher = Sha1::new();

    f.read_to_end(&mut buffer).unwrap();
    hasher.update(&buffer);
    let sha_val = hasher.digest().to_string();
    Ok(sha_val)
}

pub fn digest_sha512b64(filepath: &Path) -> Result<String, Error> {
    let mut f = File::open(filepath).ok().expect("Failed to read file");
    let mut buffer = Vec::new();
    let mut hasher = Sha512::new();

    f.read_to_end(&mut buffer).unwrap();
    hasher.input(& buffer);

    let sha_val = base64::encode(&hasher.result()).to_string();
    Ok(sha_val)
}

pub fn digest_md5(filepath: &Path) -> Result<String, Error> {
    let mut f = File::open(filepath).ok().expect("Failed to open python package for digest");
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).expect("Failed to read python package into buffer");
    let md5_val = md5::compute(buffer);
    Ok(format!("{:x}", md5_val))
}

// founds the right encoder based matching columns in DigestExtTable
// returns None when filetype is unsupported, otherwise list of all matched algos
pub fn digest_file(ext_table: &DigestExtTable, filepath: &Path) -> Option<Vec<ProductSHA>> {
    if filepath.is_dir(){ return None; }

    let opt_ext = filepath.extension();
    if opt_ext.is_none() { return None; } //when hidden file or file has no extensions

    let file_ext  = opt_ext.unwrap().to_str().unwrap_or("");
    let filepath_ = filepath.clone();//keep copy for debugging purpose
    let path_txt  = filepath.to_str().unwrap_or("").to_string();
    let mut shas: Vec<ProductSHA> = Vec::new();

    if ext_table.is_md5(file_ext.to_string()) {
        if let Some(md5_val) = digest_md5(filepath).ok() {
            shas.push(ProductSHA {
                packaging: "".to_string(),
                method: "md5".to_string(),
                value: md5_val,
                filepath: Some(path_txt.clone())
            });
        }
    }

    if ext_table.is_sha1(file_ext.to_string()) {
        if let Some(sha_val) = digest_sha1(filepath).ok() {
            shas.push(ProductSHA {
                packaging: "".to_string(),
                method: "sha1".to_string(),
                value: sha_val,
                filepath: Some(path_txt.clone())
            });
        }
    }

    if ext_table.is_sha512(file_ext.to_string()) {
        if let Some(sha_val) = digest_sha512b64(filepath).ok() {
            shas.push(ProductSHA {
                packaging: "".to_string(),
                method: "sha512".to_string(),
                value: sha_val,
                filepath: Some(path_txt)
            });
        }
    }

    if shas.len() > 0 {
        Some(shas)
    } else {
        None
    }

}



