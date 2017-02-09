extern crate sha2;
extern crate base64;
extern crate sha1;

use sha1::{Sha1};
use sha2::{Sha512, Digest};
use base64::{encode};

use std::io::prelude::*;
use std::path::Path;
use std::io;
use std::fs::{self, File};
use std::process;
use std::env;

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

fn walk_recursive_path(dir: &Path) -> io::Result<()>  {

    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();

            if path.is_dir() {
                walk_recursive_path(&path);
            } else {
                print_file_sha(&path, dispatch_encoder(&path));
            }
        }
    }

    Ok(())
}

fn print_file_sha(file_path: &Path, file_sha: Option<String>){

    match file_sha {
        Some(res) => println!("{:?},{}", file_path, res),
        None      => ()
    }
}

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Missing folder.");
        process::exit(0);
    }

    let ref path = args[1];
    let dir = Path::new(path);
    println!("Scanning: {}", path);
    walk_recursive_path(dir);
}
