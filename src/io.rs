extern crate csv;
extern crate rustc_serialize;

use std;
use std::io::{Error, ErrorKind};
use std::path::{PathBuf};
use rustc_serialize::{Encodable};


pub trait IOWriter {
    fn write_rows<I, T>(&self, rows: I) -> Result<u32, Error>
        where I: Iterator<Item=Vec<T>>, T: Encodable;
}

pub trait IOReader {
    fn read_all(&self) -> Result< Vec<Vec<String>>, Error>;
}

//--STDOUTWriter

pub struct StdOutWriter {

}

impl StdOutWriter {
    pub fn new() -> StdOutWriter {
        StdOutWriter {}
    }
}

impl IOWriter for StdOutWriter {
    fn write_rows<I, T>(&self, rows: I) -> Result<u32, Error>
        where I:Iterator<Item=Vec<T>>, T:Encodable {
        let mut n = 0u32;
        let mut wtr = csv::Writer::from_memory();
        for row in rows {
            wtr.encode(row).unwrap();
            n += 1;
        }

        println!("{}", wtr.as_string());
        Ok(n)
    }
}

//-- CSVWriter
pub struct CSVWriter {
    filepath: std::path::PathBuf
}

impl CSVWriter {
    pub fn new(the_path: String) -> CSVWriter {
        CSVWriter {filepath: PathBuf::from(the_path.clone().as_str()) }
    }
}

impl IOWriter for CSVWriter {

    fn write_rows<I, T>(&self, rows: I)  -> Result<u32, Error>
        where I:Iterator<Item=Vec<T>>, T:Encodable {

        let mut n = 0u32;
        let mut wtr = csv::Writer::from_file(&self.filepath).expect("Failed to open output file");
        wtr.flush().expect("Failed to flush CSVWriter");

        for row in rows {
            wtr.encode(row).unwrap();
            n += 1;
        }

        Ok(n)
    }
}

// CSVReader
pub struct CSVReader {
    filepath: std::path::PathBuf
}

impl CSVReader {
    pub fn new(the_path: String) -> CSVReader {
        CSVReader { filepath: PathBuf::from(the_path.clone().as_str()) }
    }
}

impl IOReader for CSVReader {

    fn read_all(&self) -> Result< Vec<Vec<String>>, Error> {

        let mut rows: Vec<_> = vec![];
        let mut rdr = csv::Reader::from_file(& self.filepath).expect("Failed to open CSV file for read");

        for row in rdr.records() {
            if let Some(items) = row.ok() {
                rows.push(items)
            }
        };

        Ok(rows)
    }
}
