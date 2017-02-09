
extern crate sha2;
extern crate base64;
extern crate sha1;
extern crate getopts;

use getopts::Options;
use std::path::Path;
use std::process;
use std::env;

mod checker;


fn do_scan_task(dir_txt: &str) {
    println!("Scanning: {}", dir_txt);
    let dir = Path::new(dir_txt);

    checker::walk_recursive_path(dir);
}

fn show_usage(program_name: &str, opts: Options){
    let brief = format!("usage: {} scan DIRECTORY_PATH [options]", program_name);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut opts = Options::new();

    //register options
    opts.optopt("o", "output", "specifies the name of output file", "FILENAME");
    opts.optflag("h", "help", "shows usage help");

    //parse command-line arguments
    let matches = match opts.parse(&args[1..]){
        Ok(m)   => { m },
        Err(f)  => { panic!(f.to_string()) }
    };

    //did user asked to see help menu
    if matches.opt_present("h") {
        show_usage(&program_name, opts);
        return;
    }

    //extract input arguments
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty(){
        matches.free[0].clone()
    } else {
        show_usage(&program_name, opts);
        return;
    };


    do_scan_task(&input);
}
