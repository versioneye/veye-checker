extern crate getopts;
extern crate veye_checker;

use getopts::Options;
use std::path::Path;
use std::error::Error;
use std::env;

use veye_checker::io::{IOWriter, IOReader};
use veye_checker::{product, api, checker, io, configs};
use product::RowSerializer;


fn show_usage(program_name: &str, opts: Options) -> Result<bool, String> {
    let brief = format!(r#"
        usage:
            {} scan DIRECTORY_PATH -o OUTPUT_FILE
            {} lookup FILE_SHA -a API_TOKEN
            {} lookup_csv SHA_FILE_PATH -o OUTPUT_FILE -a API_TOKEN
        "#, program_name, program_name, program_name
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
        show_usage(&program_name, opts).unwrap();
        return;
    }

    if matches.free.len() < 1 {
        println!("Error: Subcommand is unspecified");
        show_usage(&program_name, opts).unwrap();
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

    println!("Scanning: {}", dir_txt);
    let dir = Path::new(&dir_txt);
    let rows = match checker::scan_dir(dir, 0) {
        Ok(vals) => vals,
        Err(_)   => {
            println!("Failed to scan folder {}", &dir_txt);
            vec![]
        }
    };

    if let Some(outpath) = matches.opt_str("o") {
        let path_str = outpath.clone();
        let wtr = io::CSVWriter::new(path_str.clone());

        println!("Will dump results into file... {}", &path_str);
        wtr.write_rows(rows.into_iter()).unwrap();

    } else {
        println!("No output file were defined");
        let wtr = io::StdOutWriter::new();
        wtr.write_rows(rows.into_iter()).unwrap();
    }

    Ok(true)
}

fn do_lookup_task(matches: &getopts::Matches) -> Result<bool, String> {

    let file_sha = if matches.free.len() != 2 {
        panic!("lookup command misses SHA-code");
    } else {
        matches.free[1].clone()
    };

    let mut global_configs = configs::read_configs();
    //override global configs when use attached commandline key
    if global_configs.api.key.is_none() && matches.opt_str("a").is_some() {
        global_configs.api.key = matches.opt_str("a")
    };

    if global_configs.api.key.is_none() {
        panic!(
            "Missing API key: SET env var VERSIONEYE_API_KEY, or use -a param, or use veye_checker.toml"
        );
    };

    let out_filepath = matches.opt_str("o");
    match api::fetch_product_details_by_sha(&global_configs.api, &file_sha.clone()) {
        Ok(m) => {
            let mut rows = vec![];
            rows.push(m.to_fields());
            for r in m.to_rows() { rows.push(r); }

            if out_filepath.is_none() {
                let wtr = io::StdOutWriter::new();
                wtr.write_rows(rows.into_iter()).iter();

            } else {
                let out_fp = out_filepath.unwrap();
                let wtr = io::CSVWriter::new(out_fp.clone());

                wtr.write_rows( rows.into_iter() ).unwrap();
                println!("Dumped result into {}", out_fp.clone());
            }

        },
        Err(e)  => println!("No product info for sha {} - {}", file_sha, e.description())
    }

    Ok(true)
}

fn do_lookup_csv_task(matches: &getopts::Matches) -> Result<bool, String> {
    let sha_results_filepath = if matches.free.len() != 2 {
        panic!("lookup_csv: no input file was specified");
    } else {
        matches.free[1].clone()
    };

    let mut  global_configs = configs::read_configs();
    //override global configs when use attached commandline key
    if global_configs.api.key.is_none() && matches.opt_str("a").is_some() {
        global_configs.api.key = matches.opt_str("a")
    };
    if global_configs.api.key.is_none() {
        panic!(
            "Missing API key: SET env var VERSIONEYE_API_KEY, or use -a param, or use veye_checker.toml"
        );
    };

    let output_path = matches.opt_str("o").expect("Missing output file");
    let rdr = io::CSVReader::new( sha_results_filepath );
    let mut csv_rows: Vec<Vec<String>> = vec![];
    let product_headers = product::ProductMatch::empty().to_fields();

    csv_rows.push(product_headers);
    for row in rdr.read_all().expect("Failed to read sha file") {
        let (file_path, file_sha) = (&row[0], &row[1]);

        let the_prod = match api::fetch_product_details_by_sha(&global_configs.api, &file_sha.clone()) {
            Ok(mut m) => {
                m.filepath = Some(file_path.clone());
                m
            },
            Err(_) => {
                println!("Failed to get product details for {}, {}", file_path.clone(), file_sha.clone());
                let mut empty_m = product::ProductMatch::empty();
                empty_m.filepath = Some(file_path.clone());
                empty_m
            }
        };

        for row in the_prod.to_rows() {
            csv_rows.push(row);
        };

    };

    let csv_writer = io::CSVWriter::new(output_path);
    csv_writer.write_rows(csv_rows.into_iter()).unwrap();

    Ok(true)
}

fn print_cmd_result(cmd_res: Result<bool, std::string::String>){
    match cmd_res {
        Ok(_) => println!("Done!"),
        Err(e)  => println!("Failed to finish the task: {}", e)
    };
}
