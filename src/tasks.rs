extern crate csv;

use std::path::PathBuf;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::error::Error;
use std::io;

use product::{self, ProductSHA, ProductMatch};
use product::RowSerializer;
use configs;
use api;
use checker;



pub fn start_path_scanner(dir_path: PathBuf)
    -> (Receiver<product::ProductSHA>, thread::JoinHandle<Result<(), io::Error>> ) {
    let (sender, receiver) = channel::<ProductSHA>();
    let handle = thread::spawn(move || {
        if let Some(shas) = checker::scan_dir(&dir_path, 0).ok() {
            for sha in shas.into_iter() {
                if sender.send(sha).is_err(){
                    println!("start_path_scanner: failed send ProductSHA");
                    break
                }
            }
        }

        Ok(())
    });

    (receiver, handle)
}

pub fn start_sha_fetcher(api_configs: configs::ApiConfigs, sha_ch:  Receiver<ProductSHA>)
    -> (Receiver<product::ProductMatch>, thread::JoinHandle<io::Result<()>>) {

    let (sender, receiver) = channel::<ProductMatch>();
    let handle = thread::spawn(move || {
        for sha in sha_ch.into_iter() {
            let sha_code = sha.value.clone();
            let prod = match api::fetch_product_details_by_sha(&api_configs, sha_code.as_str()) {
                Ok(mut m) => {
                    m.sha = Some(sha); //attach original sha document to have filepath data
                    m
                },
                Err(e) => {
                    //use empty product, so non-matched products will show up in output file
                    let mut m = ProductMatch::empty();
                    m.sha = Some(sha);
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

pub fn start_csv_writer(outpath: PathBuf, product_ch: Receiver<ProductMatch>)
    -> thread::JoinHandle< Result<(), csv::Error> > {

    thread::spawn(move || {
        let mut n = 0u32;
        let mut wtr = csv::Writer::from_file(outpath).expect("Failed to open output file");

        for product in product_ch.into_iter() {
            if n == 0 {
                wtr.encode(product.to_fields()).unwrap();
            };

            for row in product.to_rows().into_iter() {
                wtr.encode(row).unwrap();
            }

            n += 1;
        }

        Ok(())
    })

}

pub fn start_stdio_writer(product_ch: Receiver<ProductMatch>)
    -> thread::JoinHandle<Result<(), csv::Error >> {

    thread::spawn(move || {
        let mut n = 0u32;

        for product in product_ch.into_iter() {
            let mut wtr = csv::Writer::from_memory();

            if n == 0 {
                wtr.encode(product.to_fields()).unwrap();
            }

            for row in product.to_rows().into_iter() {
                wtr.encode(row).unwrap();
            }

            println!("{}", wtr.as_string());
            n += 1;
        }

        Ok(())
    })
}