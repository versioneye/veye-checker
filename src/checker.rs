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

pub fn scan_dir(dir: &Path, outpath: Option<&Path> ) -> Result<bool, io::Error>  {

    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();

            if path.is_dir() {
                scan_dir(&path, outpath);
            } else if path.is_file() {
                print_sha(&path, dispatch_encoder(&path), outpath);
            } else {
                println!("Going to ignore {:?}", path.to_str());
            }
        }
    }

    Ok(true)
}

fn print_sha(file_path: &Path, file_sha: Option<String>, outpath: Option<&Path>){

    match file_sha {
        Some(res) => {
            if outpath.is_some() {
                print_sha_file(&file_path, &res, &outpath.unwrap())
            } else {
                print_sha_screen(&file_path, &res)
            }
        },
        None      => ()
    }
}

fn print_sha_screen(file_path: &Path, file_sha: &str){
    println!("{:?},{}", file_path, file_sha);
}

fn print_sha_file(file_path: &Path, file_sha: &str, outpath: &Path){
    let mut f = OpenOptions::new().write(true).append(true).open(outpath).expect("Failed to open output file");
        //.ok().expect("Failed to write line into output file");
    let line = format!("{:?},{}\n", file_path, file_sha);
    f.write_all( line.as_bytes() ).expect("Failed to write sha line to file");
    f.sync_all().expect("Failed to sync buffer into output file");

}
