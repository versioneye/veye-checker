extern crate getopts;
extern crate veye_checker;

use getopts::Options;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::env;

use veye_checker::{product, api, checker, configs, tasks};
use product::RowSerializer;


fn show_usage(program_name: &str, opts: Options) -> Result<bool, String> {
    let brief = format!(r#"
        usage:
            {} resolve DIRECTORY_TO_SCAN -o OUTPUT_FILE -a API_TOKEN
            {} shas DIRECTORY_PATH -o OUTPUT_FILE
            {} lookup FILE_SHA -a API_TOKEN
        "#, program_name, program_name, program_name
    );

    println!("{}", opts.usage(&brief));
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
        "resolve"       => do_resolve_task(&matches),
        "shas"          => do_shas_task(&matches),
        "lookup"        => do_lookup_task(&matches),
        _               => show_usage(&program_name, opts)
    };

    print_cmd_result(cmd_res);
}

fn do_resolve_task(matches: &getopts::Matches) -> Result<bool, String> {
    let dir_txt = if matches.free.len() != 2 {
        panic!("resolve tasks requires target folder".to_string());
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

    // execute command pipeline
    let dir = PathBuf::from(&dir_txt);
    let (sha_ch, h1) = tasks::start_path_scanner(dir);
    let (product_ch, h2) = tasks::start_sha_fetcher(global_configs.api.clone(), sha_ch);
    let h3 = match matches.opt_str("o") {
        Some(out_path) => {
            let out_path = PathBuf::from(out_path);
            tasks::start_product_csv_writer(out_path, product_ch)
        },
        None => tasks::start_product_stdio_writer(product_ch)
    };

    h1.join().expect("resolve_task: failed to finish scan task");
    h2.join().expect("resolve_task: failed to finish SHA fetcher task");
    h3.join().expect("resolve_task: failed to dump all the products into output");

    Ok(true)

}


fn do_shas_task(matches: &getopts::Matches) -> Result<bool, String> {
    //extract input arguments
    let dir_txt = if matches.free.len() != 2 {
        panic!("scan command misses a path to folder".to_string());
    } else {
        matches.free[1].clone()
    };

    let dir = PathBuf::from(&dir_txt);
    let (sha_ch, h1) = tasks::start_path_scanner(dir);
    let h2 = match matches.opt_str("o") {
        Some(outfile_path) => {
            let outpath = PathBuf::from(&outfile_path);
            tasks::start_sha_csv_writer(outpath, sha_ch)
        },
        None => tasks::start_sha_stdio_writer(sha_ch)

    };

    h1.join().expect("shas: failed to scan file digests");
    h2.join().expect("shas: failed to print results into output");

    Ok(true)
}

fn do_lookup_task(matches: &getopts::Matches) -> Result<bool, String> {

    let file_sha = if matches.free.len() != 2 {
        panic!("lookup command misses SHA-code");
    } else {
        matches.free[1].clone()
    };

    let mut global_configs = configs::read_configs();
    //override api key when it was specified via -a flag
    if global_configs.api.key.is_none() && matches.opt_str("a").is_some() {
        global_configs.api.key = matches.opt_str("a")
    };

    if global_configs.api.key.is_none() {
        panic!(
            "Missing API key: SET env var VERSIONEYE_API_KEY, or use -a param, or use veye_checker.toml"
        );
    };

    let shas = vec![
        product::ProductSHA::from_sha(file_sha.clone().to_string())
    ];
    let (sha_ch, h1) = tasks::start_sha_publisher(shas);
    let (prod_ch, h2) = tasks::start_sha_fetcher(global_configs.api.clone(), sha_ch);
    let h3 = match matches.opt_str("o") {
        Some(outfile_path) => {
            let outpath = PathBuf::from(&outfile_path);
            tasks::start_product_csv_writer(outpath, prod_ch)
        },
        None => tasks::start_product_stdio_writer(prod_ch)
    };

    h1.join().expect("lookup: failed to prepare sha value for request");
    h2.join().expect("lookup: failed to fetch product details by sha value");
    h3.join().expect("lookup: failed to output product details");

    Ok(true)
}


fn print_cmd_result(cmd_res: Result<bool, std::string::String>){
    match cmd_res {
        Ok(_) => println!("Done!"),
        Err(e)  => println!("Failed to finish the task: {}", e)
    };
}
