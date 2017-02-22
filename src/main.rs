
extern crate sha2;
extern crate base64;
extern crate sha1;
extern crate getopts;
extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate csv;

use getopts::Options;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::process;
use std::env;
use std::error::Error;
use std::rc::Rc;
use std::borrow::Borrow;

mod checker;
mod api;
mod product;

use product::CSVSerializer;

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
            {} lookup FILE_SHA -a API_TOKEN
            {} lookup_csv SHA_FILE_PATH -o OUTPUT_FILE -a API_TOKEN
        "#,
        program_name, program_name, program_name
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
    let cmd_res = match command.as_ref() {
        "scan"          => do_scan_task(&matches),
        "lookup"        => do_lookup_task(&matches),
        "lookup_csv"    => do_lookup_csv_task(&matches),
        _               => show_usage(&program_name, opts)
    };

    print_cmd_result(cmd_res);
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
        checker::scan_dir(dir, Some(&out_dir));

    } else {
        println!("No output file were defined");
        checker::scan_dir(dir, None);
    }
    Ok(true)
}

fn fetch_product_details(file_sha: &str, api_key: &str) -> Result<product::ProductMatch, std::io::Error> {
    println!("Going to checkup product by SHA: {}", file_sha);
    let sha_res = api::fetch_product_by_sha(&file_sha, &api_key.clone());


    match sha_res {
        Ok(m) => {
            println!("Going to check product details by matched SHA result");
            let sha = m.sha.expect("No product sha from SHA result");
            let product = m.product.expect("No product info from SHA result");
            match api::fetch_product( &product.language, &product.prod_key, &product.version, &api_key ) {
                Ok(mut m) => {
                    m.sha = Some(sha);
                    Ok(m)
                },
                Err(e) => Err(e)
            }

        },
        Err(e) => Err(e)
    }
}

fn do_lookup_task(matches: &getopts::Matches) -> Result<bool, String> {

    let file_sha = if matches.free.len() != 2 {
        panic!("lookup command misses SHA-code");

    } else {
        matches.free[1].clone()
    };

    let api_key = matches.opt_str("a").expect("Missing API_KEY!");
    let out_filepath = matches.opt_str("o");

    match fetch_product_details(&file_sha.clone(), &api_key) {
        Ok(m) => {
            if out_filepath.is_none() {
                println!("{}{}", m.to_csv_header(), m.to_csv() )
            } else {
                let out_fp = out_filepath.unwrap();
                let mut wtr = File::create(out_fp).expect("Failed to open outout file");
                wtr.write_all(& m.to_csv_header().into_bytes());
                wtr.write_all(& m.to_csv().into_bytes());
                wtr.sync_data();

                println!("Dumped result into specified file;");
            }
        },
        Err(e)  => println!("No product info for sha {}", file_sha)
    }

    Ok(true)
}

fn do_lookup_csv_task(matches: &getopts::Matches) -> Result<bool, String> {
    let sha_results_filepath = if matches.free.len() != 2 {
        panic!("lookup_csv: no input file was specified");
    } else {
        matches.free[1].clone()
    };

    let api_key = matches.opt_str("a").expect("Missing API_KEY!");
    let output_path = matches.opt_str("o").expect("Missing output file");

    let mut rdr = csv::Reader::from_file(
        sha_results_filepath.clone()
    ).expect(format!("Failed to read SHA file from {}", sha_results_filepath).as_ref());

    let mut csv_rows = vec![];

    for row in rdr.decode() {

        let (file_path, file_sha): (String, String) = row.unwrap();
        match fetch_product_details(&file_sha.clone(), &api_key){
            Ok(mut m) => {
                m.filepath = Some(file_path.clone());
                if csv_rows.len() == 0 {
                    csv_rows.push(m.to_csv_header());
                }

                csv_rows.push(m.to_csv());
            },
            Err(e) => {
                println!("Failed to get product details for {}, {}", file_path.clone(), file_sha.clone());
                let empty_m = product::ProductMatch {
                    sha: None,
                    product: None,
                    url: None,
                    licenses: vec![],
                    n_vulns: 0,
                    filepath: Some(file_path.clone())
                };

                if csv_rows.len() == 0 {
                    csv_rows.push(empty_m.to_csv_header());
                }
                csv_rows.push(empty_m.to_csv());
            }
        }

    };

    let mut wtr = File::create(output_path).expect("Failed to open output file");
    for row in csv_rows {
        wtr.write_all(& row.into_bytes());
    };
    wtr.sync_data();

    Ok(true)
}


fn print_cmd_result(cmd_res: Result<bool, std::string::String>){
    match cmd_res {
        Ok(_) => println!("Done!"),
        Err(e)  => println!("Failed to finish the task: {}", e)
    };
}
