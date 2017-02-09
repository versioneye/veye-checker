
extern crate sha2;
extern crate base64;
extern crate sha1;

use std::path::Path;
use std::process;
use std::env;

mod checker;

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Missing folder.");
        process::exit(0);
    }

    let ref path = args[1];
    let dir = Path::new(path);
    println!("Scanning: {}", path);
    checker::walk_recursive_path(dir);
}
