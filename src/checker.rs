use sha1::{Sha1};
use sha2::{Sha512, Digest};
use base64::{encode};

use std::io::prelude::*;
use std::path::Path;
use std::io;
use std::fs::{self, File};
use std::fs::OpenOptions;

fn encode_jar(filepath: &Path) -> String {
    //println!("Going to read jar file from {:?}", filepath);

    let mut f = File::open(filepath).ok().expect("Failed to read file");
    let mut buffer = Vec::new();

    let mut hasher = Sha1::new();

    f.read_to_end(&mut buffer);
    hasher.update(& buffer);
    hasher.digest().to_string()
}

fn encode_nuget(filepath: &Path) -> String {
    //println!("Going to read nupkg file from {:?}", filepath);

    let mut f = File::open(filepath).ok().expect("Failed to read file");
    let mut buffer = Vec::new();

    let mut hasher = Sha512::new();

    f.read_to_end(&mut buffer);
    hasher.input(& buffer);

    encode(&hasher.result()).to_string()
}

// founds the right encoder based on the filepath
// returns None when filetype is unsupported
fn dispatch_encoder(filepath: &Path) -> Option<String> {
    let opt_ext = filepath.extension();
    if opt_ext.is_none() { return None; } //when hidden file or file has no extensions

    let file_ext = opt_ext.unwrap().to_str().unwrap_or("");

    match file_ext {
        "nupkg" => Some(encode_nuget(filepath)),
        "jar"   => Some(encode_jar(filepath)),
        _       => None
    }
}

pub fn scan_dir(dir: &Path, depth: u32) -> Result< Vec<Vec<String>>, io::Error>  {
    let mut rows = vec![];
    if depth == 0 {
        rows.push(vec!["file_path".to_string(), "file_sha".to_string()]);
    }

    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();

            if path.is_dir() {
                match scan_dir(&path, depth + 1){
                    Ok(mut dir_rows) => rows.append(&mut dir_rows),
                    Err(e) => println!("Failed to scan folder {:?}", path)
                };

            } else if path.is_file() {
                //dont append files without sha
                if let Some(file_sha) = dispatch_encoder(&path) {
                    let file_name = path.to_str().unwrap_or("unknown_path").to_string();
                    rows.push( vec![file_name, file_sha] )
                }

            } else {
                println!("Going to ignore {:?}", path.to_str());
            }
        }
    }

    Ok(rows)
}
