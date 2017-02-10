
extern crate sha2;
extern crate base64;
extern crate sha1;
extern crate getopts;

use getopts::Options;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::process;
use std::env;

mod checker;

fn init_out_file(outfile_path: &Path) -> Result<bool, std::io::Error> {
    //it creates a new file or truncates existing one
    let mut f = File::create( outfile_path ).ok().expect("Failed to create output file");
    try!(f.write_all(b"file_path,package_sha\n"));
    try!(f.sync_all());

    Ok(true)
}

fn do_scan_task(dir_txt: &str, outpath: Option<String>) -> Result<bool, std::io::Error>{
    println!("Scanning: {}", dir_txt);
    let dir = Path::new(dir_txt);

    if outpath.is_some() {
        println!("Will dump results into file...");
        let path_str = outpath.unwrap();
        let out_dir =  Path::new(&path_str);
        let res = init_out_file(&out_dir);
        match res {
            Ok(_)   => checker::scan_dir(dir, Some(&out_dir)),
            Err(e)  => Err(e)
        };

    } else {
        println!("No output file were defined");
        checker::scan_dir(dir, None);
    }
    Ok(true)
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

    // fire the task
    match do_scan_task(&input, output){
        Ok(res) => println!("Done!"),
        Err(e)  => println!("Failed to scan folders")
    };
}
