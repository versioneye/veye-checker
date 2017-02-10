
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



fn show_usage(program_name: &str, opts: Options) -> Result<bool, String> {
    let brief = format!(r#"
        usage:
            {} scan DIRECTORY_PATH -o OUTPUT_FILE
            {} lookop FILE_SHA -a API_TOKEN
        "#,
        program_name, program_name
    );
    print!("{}", opts.usage(&brief));
    Ok(true)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut opts = Options::new();

    //register options
    opts.optopt("o", "output", "specifies the name of output file", "FILENAME");
    opts.optopt("a", "auth", "specifies api-key for API calls", "API_TOKEN");
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

    if matches.free.len() < 1 {
        println!("Error: Subcommand is unspecified");
        show_usage(&program_name, opts);
        return;
    }

    let command = matches.free[0].clone();
    //println!("User asked command: {}", command);

    match command.as_ref() {
        "scan"      => do_scan_task(&matches),
        "lookup"    => do_lookup_task(&matches),
        _           => show_usage(&program_name, opts)
    };

    println!("Done")
}


fn do_scan_task(matches: &getopts::Matches) -> Result<bool, String> {
    //extract input arguments
    let dir_txt = if matches.free.len() != 2 {
        panic!("scan command misses a path to folder".to_string());
    } else {
        matches.free[1].clone()
    };
    let outpath = matches.opt_str("o");

    println!("Scanning: {}", dir_txt);
    let dir = Path::new(&dir_txt);

    if outpath.is_some() {
        println!("Will dump results into file...");
        let path_str = outpath.unwrap();
        let out_dir =  Path::new(&path_str);
        let res = init_out_file(&out_dir);
        let cmd_res = match res {
            Ok(_)   => checker::scan_dir(dir, Some(&out_dir)),
            Err(e)  => Err(e)
        };
        print_cmd_result(cmd_res);

    } else {
        println!("No output file were defined");
        checker::scan_dir(dir, None);
    }
    Ok(true)
}

fn do_lookup_task(matches: &getopts::Matches) -> Result<bool, String> {

    let file_sha = if matches.free.len() != 2 {
        panic!("lookup command misses SHA-code");

    } else {
        matches.free[1].clone()
    };
    let api_key = matches.opt_str("a");

    println!("Going to checkup product details by SHA: #{}", file_sha);

    Ok(true)
}

fn print_cmd_result(cmd_res: Result<bool, std::io::Error>){
    match cmd_res {
        Ok(res) => println!("Done!"),
        Err(e)  => println!("Failed to finish the task")
    };
}