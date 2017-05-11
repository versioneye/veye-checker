extern crate getopts;
extern crate veye_checker;

use getopts::Options;
use std::path::PathBuf;
use std::env;

use veye_checker::{product, configs, tasks, digest_ext_table};

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

    //register options flags
    opts.optopt("o", "output", "specifies the name of output file", "FILENAME");
    opts.optopt("a", "auth", "specifies the api-key for API calls", "API_TOKEN");
    opts.optopt("c", "config", "specifies the filepath to lookup configfile", "FILEPATH");
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
        println!("Error: missing the subcommand");
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
    let mut global_configs = configs::read_configs(matches.opt_str("c"));

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
    let ext_table = digest_ext_table::DigestExtTable::default();
    let dir = PathBuf::from(&dir_txt);
    let (sha_ch, h1) = tasks::start_path_scanner(ext_table, dir);
    let (product_ch, h2) = tasks::start_sha_fetcher(global_configs.clone(), sha_ch);
    let h3 = match matches.opt_str("o") {
        Some(out_path) => {
            let out_path = PathBuf::from(out_path);
            tasks::start_product_csv_writer(out_path, global_configs.csv.clone(), product_ch)
        },
        None => tasks::start_product_stdio_writer(global_configs.csv.clone(), product_ch)
    };

    h1.join().expect("resolve_task: failed to finish scan task").unwrap();
    h2.join().expect("resolve_task: failed to finish SHA fetcher task").unwrap();
    h3.join().expect("resolve_task: failed to dump all the products into output").unwrap();

    Ok(true)

}


fn do_shas_task(matches: &getopts::Matches) -> Result<bool, String> {
    let global_configs = configs::read_configs(matches.opt_str("c"));

    //extract input arguments
    let dir_txt = if matches.free.len() != 2 {
        panic!("scan command misses a path to folder".to_string());
    } else {
        matches.free[1].clone()
    };


    let dir = PathBuf::from(&dir_txt);
    let ext_table = digest_ext_table::DigestExtTable::default();
    let (sha_ch, h1) = tasks::start_path_scanner(ext_table, dir);
    let h2 = match matches.opt_str("o") {
        Some(outfile_path) => {
            let outpath = PathBuf::from(&outfile_path);
            tasks::start_sha_csv_writer(outpath, global_configs.csv.clone(), sha_ch)
        },
        None => tasks::start_sha_stdio_writer(global_configs.csv.clone(), sha_ch)

    };

    h1.join().expect("shas: failed to scan file digests").unwrap();
    h2.join().expect("shas: failed to print results into output").unwrap();

    Ok(true)
}

fn do_lookup_task(matches: &getopts::Matches) -> Result<bool, String> {

    let file_sha = if matches.free.len() != 2 {
        panic!("lookup command misses SHA-code");
    } else {
        matches.free[1].clone()
    };

    let mut global_configs = configs::read_configs(matches.opt_str("c"));
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
    let (prod_ch, h2) = tasks::start_sha_fetcher(global_configs.clone(), sha_ch);
    let h3 = match matches.opt_str("o") {
        Some(outfile_path) => {
            let outpath = PathBuf::from(&outfile_path);
            tasks::start_product_csv_writer(outpath, global_configs.csv.clone(), prod_ch)
        },
        None => tasks::start_product_stdio_writer(global_configs.csv.clone(), prod_ch)
    };

    h1.join().expect("lookup: failed to prepare sha value for request").unwrap();
    h2.join().expect("lookup: failed to fetch product details by sha value").unwrap();
    h3.join().expect("lookup: failed to output product details").unwrap();

    Ok(true)
}


fn print_cmd_result(cmd_res: Result<bool, std::string::String>){
    match cmd_res {
        Ok(_) => println!("Done!"),
        Err(e)  => println!("Failed to finish the task: {}", e)
    };
}
