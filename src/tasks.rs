extern crate csv;

use std::fs;
use std::vec;
use std::path::PathBuf;
use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::io::{self, ErrorKind};
use std::error::Error;

use walkdir::WalkDir;

use product::{self, ProductSHA, ProductMatch};
use product::RowSerializer;
use configs;
use api;
use checker;
use digest_ext_table::DigestExtTable;

pub fn start_path_scanner(ext_table: DigestExtTable, dir: PathBuf)
    -> (Receiver<ProductSHA>, thread::JoinHandle<Result<(), io::Error>> ) {

    let (sender, receiver) = channel::<ProductSHA>();
    let handle = thread::spawn(move || {
        if dir.exists() == false {
            return Err(
              io::Error::new(ErrorKind::Other, "Scannable folder doesnt exists")
            );
        }

        for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()){

            if let Some(shas) = checker::digest_file(&ext_table, &entry.path()) {
                for sha in shas.into_iter(){
                    if sender.send(sha).is_err() {
                        println!(
                            "start_path_scanner2: failed to send ProductSHA for {}",
                            entry.path().display()
                        );
                        break
                    }
                }
            }
        }

        Ok(())
    });

    (receiver, handle)
}

//pumps vector of SHAs onto sha channel
pub fn start_sha_publisher(shas: Vec<ProductSHA>)
    -> (Receiver<ProductSHA>, thread::JoinHandle<Result<(), io::Error>>) {

    let (sender, receiver) = channel::<ProductSHA>();
    let handle = thread::spawn(move || {
        for sha in shas.into_iter() {
            if sender.send(sha).is_err() {
                println!("start_sha_publisher: failed to send ProductSHAs");
                break
            }
        }

        Ok(())
    });

    (receiver, handle)
}

//pumps each item of productMatch vector onto product channel
//used to simplify testing
pub fn start_product_match_publisher(prod_matches: Vec<ProductMatch>)
    -> (Receiver<ProductMatch>, thread::JoinHandle<Result<(), io::Error>>) {

    let (sender, receiver) = channel::<ProductMatch>();
    let handle = thread::spawn(move || {
        for prod_match in prod_matches.into_iter() {
            if sender.send(prod_match).is_err() {
                println!("start_product_match_publisher: failed to pipe ProductMatch onto channel");
                break
            }
        }

        Ok(())
    });

    (receiver, handle)
}

pub fn start_sha_fetcher(configs: configs::Configs, sha_ch:  Receiver<ProductSHA>)
    -> (Receiver<product::ProductMatch>, thread::JoinHandle<io::Result<()>>) {

    let (sender, receiver) = channel::<ProductMatch>();
    let handle = thread::spawn(move || {
        for sha in sha_ch.into_iter() {
            let sha_code = sha.value.clone();
            let prod = match api::fetch_product_details_by_sha(&configs, sha_code.as_str()) {
                Ok(mut m) => {
                    m.sha = Some(sha); //attach original sha document to have filepath data
                    m
                },
                Err(e) => {
                    //use empty product, so non-matched products will show up in output file
                    let mut m = ProductMatch::empty();
                    m.sha = Some(sha);
                    m.error = Some(e.description().to_string()); //attach error message
                    m
                }
            };

            if sender.send(prod).is_err(){
                break;
            }
        }

        Ok(())
    });

    (receiver, handle)
}

fn init_csv_file_writer(outpath: PathBuf, csv_configs: configs::CSVConfigs)
    ->  csv::Writer<fs::File> {
    let mut wtr = csv::Writer::from_file(outpath).expect("Failed to open output file");

    if let Some(sep) = csv_configs.separator {
        let ch = if sep.len() > 0 {
            sep.as_bytes()[0]
        } else {
            ";".as_bytes()[0]
        };

        wtr = wtr.delimiter(ch);
    }

    if let Some(quote) = csv_configs.quote {
        let ch2 = if quote.len() > 0 {
            quote.as_bytes()[0]
        } else {
            "\"".as_bytes()[0]
        };

        wtr = wtr.quote(ch2);
    }

    if let Some(is_flex) = csv_configs.flexible { wtr = wtr.flexible(is_flex); };

    wtr
}

fn init_csv_stdio_writer(csv_configs: configs::CSVConfigs) -> csv::Writer<vec::Vec<u8>> {
    let mut wtr = csv::Writer::from_memory();

    if let Some(sep) = csv_configs.separator {
        let ch = if sep.len() > 0 {
            sep.as_bytes()[0]
        } else {
            ";".as_bytes()[0]
        };

        wtr = wtr.delimiter(ch);
    }

    if let Some(quote) = csv_configs.quote {
        let ch2 = if quote.len() > 0 {
            quote.as_bytes()[0]
        } else {
            "\"".as_bytes()[0]
        };

        wtr = wtr.quote(ch2);
    }

    if let Some(is_flex) = csv_configs.flexible { wtr = wtr.flexible(is_flex); };

    wtr
}

pub fn start_product_csv_writer(
    outpath: PathBuf, csv_configs: configs::CSVConfigs, product_ch: Receiver<ProductMatch>
) -> thread::JoinHandle< Result<(), csv::Error> > {

    thread::spawn(move || {
        let mut n = 0u32;
        let mut wtr = init_csv_file_writer(outpath, csv_configs);

        println!();
        for product in product_ch.into_iter() {
            if n == 0 {
                wtr.encode(product.to_fields()).unwrap();
            };

            for row in product.to_rows().into_iter() {
                wtr.encode(row).unwrap();
            }

            print!("\rrow: {}", n + 1); //to show some progress
            n += 1;
        }

        println!();
        Ok(())
    })

}

pub fn start_product_stdio_writer(
    csv_configs: configs::CSVConfigs, product_ch: Receiver<ProductMatch>
) -> thread::JoinHandle<Result<(), csv::Error >> {

    thread::spawn(move || {
        let mut n = 0u32;

        for product in product_ch.into_iter() {
            let mut wtr = init_csv_stdio_writer(csv_configs.clone());

            if n == 0 {
                wtr.encode(product.to_fields()).unwrap();
            }

            for row in product.to_rows().into_iter() {
                wtr.encode(row).unwrap();
            }

            print!("{}", wtr.as_string());
            n += 1;
        }

        Ok(())
    })
}


pub fn start_sha_csv_writer(outpath: PathBuf, csv_configs: configs::CSVConfigs, sha_ch: Receiver<ProductSHA>)
    -> thread::JoinHandle< Result<(), csv::Error> > {

    thread::spawn(move || {
        let mut n = 0u32;
        let mut wtr = init_csv_file_writer(outpath, csv_configs);

        println!();
        for sha in sha_ch.into_iter() {
            if n == 0 {
                wtr.encode(sha.to_fields()).unwrap();
            };

            if let Some(row) = sha.to_rows().pop() {
                wtr.encode(row).unwrap();
            }

            print!("\rrow: {}", n + 1); //to show some progress
            n += 1;
        }

        println!();
        Ok(())
    })

}

pub fn start_sha_stdio_writer(csv_configs: configs::CSVConfigs, sha_ch: Receiver<ProductSHA>)
    -> thread::JoinHandle<Result<(), csv::Error >> {

    thread::spawn(move || {
        let mut n = 0u32;

        for sha in sha_ch.into_iter() {
            let mut wtr = init_csv_stdio_writer(csv_configs.clone());

            if n == 0 {
                wtr.encode(sha.to_fields()).unwrap();
            }

            if let Some(row) = sha.to_rows().pop() {
                wtr.encode(row).unwrap();
            }

            print!("{}", wtr.as_string());
            n += 1;
        }

        Ok(())
    })
}