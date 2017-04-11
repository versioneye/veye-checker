use sha1::{Sha1};
use sha2::{Sha512, Digest};
use base64::encode;
use md5;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use product::ProductSHA;

pub fn digest_jar(filepath: &Path) -> ProductSHA {
    let mut f = File::open(filepath).ok().expect("Failed to read file");
    let mut buffer = Vec::new();

    let mut hasher = Sha1::new();

    f.read_to_end(&mut buffer).unwrap();
    hasher.update(& buffer);
    let sha_val = hasher.digest().to_string();

    ProductSHA {
        packaging: "jar".to_string(),
        method: "sha1".to_string(),
        value: sha_val,
        filepath: Some(filepath.to_str().unwrap_or("").to_string())
    }
}

pub fn digest_nupkg(filepath: &Path) -> ProductSHA {
    let mut f = File::open(filepath).ok().expect("Failed to read file");
    let mut buffer = Vec::new();
    let mut hasher = Sha512::new();

    f.read_to_end(&mut buffer).unwrap();
    hasher.input(& buffer);

    let sha_val = encode(&hasher.result()).to_string();
    ProductSHA {
        packaging: "nupkg".to_string(),
        method: "sha512".to_string(),
        value: sha_val,
        filepath: Some(filepath.to_str().unwrap_or("").to_string())
    }
}

pub fn digest_pypi(filepath: &Path) -> ProductSHA {
    let mut f = File::open(filepath).ok().expect("Failed to open python package for digest");
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).expect("Failed to read python package into buffer");
    let md5_val = md5::compute(buffer);

    ProductSHA {
        packaging: "pypi".to_string(),
        method: "md5".to_string(),
        value: format!("{:x}", md5_val),
        filepath: Some(filepath.to_str().unwrap_or("").to_string())
    }
}

// founds the right encoder based on the filepath
// returns None when filetype is unsupported
pub fn digest_file(filepath: &Path) -> Option<ProductSHA> {
    if filepath.is_dir(){ return None; }

    let opt_ext = filepath.extension();
    if opt_ext.is_none() { return None; } //when hidden file or file has no extensions

    let file_ext = opt_ext.unwrap().to_str().unwrap_or("");

    match file_ext {
        "nupkg"     => Some(digest_nupkg(filepath)),
        "jar"       => Some(digest_jar(filepath)),
        "gz"        => Some(digest_pypi(filepath)),
        "whl"       => Some(digest_pypi(filepath)),
        _           => None
    }
}



