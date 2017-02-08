extern crate sha2;
extern crate base64;
extern crate regex;
extern crate sha1;

use sha1::{Sha1};
use sha2::{Sha512, Digest};
use base64::{encode};
use regex::Regex;

use std::io::prelude::*;
use std::fs::File;
use std::process;
use std::env;

fn encode_jar(filepath: &str) -> String {
    println!("Going to read jar file from {}", filepath);

    let mut f = File::open(filepath).ok().expect("Failed to read file");
    let mut buffer = Vec::new();

    let mut hasher = Sha1::new();

    f.read_to_end(&mut buffer);
    hasher.update(& buffer);
    hasher.digest().to_string()
}

fn encode_nuget(filepath: &str) -> String {
    println!("Going to read nupkg file from {}", filepath);

    let mut f = File::open(filepath).ok().expect("Failed to read file");
    let mut buffer = Vec::new();

    let mut hasher = Sha512::new();

    f.read_to_end(&mut buffer);
    hasher.input(& buffer);

    encode(&hasher.result()).to_string()
}

// founds the right encoder based on the filepath
// returns None when filetype is unsupported
fn dispatch_encoder(filepath: &str) -> Option<String> {
    let nupkg_rule  = Regex::new(r"\.nupkg\z").unwrap();
    let jar_rule    = Regex::new(r"\.jar\z").unwrap();

    if nupkg_rule.is_match(filepath) {
        println!("It's nuget file");
        Some(encode_nuget(filepath))

    } else if jar_rule.is_match(filepath) {
        println!("It's java jar file");
        Some(encode_jar(filepath))

    } else {
        println!("Unsupported filename");
        None
    }
}

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Missing filename.");
        process::exit(0);
    }

    let ref filepath = args[1];
    match dispatch_encoder(filepath){
        Some(res) => println!("sha512: {}", res),
        None      => println!("error: unsupported file type")
    }
}
