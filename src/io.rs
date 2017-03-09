extern crate csv;
extern crate rustc_serialize;

use std;
use rustc_serialize::Encodable;
use std::fmt::Debug;


pub trait IOWriter {
    fn write_rows<I, T>(&self, rows: I) -> Result<u32, std::io::Error>
        where I: Iterator<Item=Vec<T>>, T: Encodable;
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
    fn write_rows<I, T>(&self, rows: I) -> Result<u32, std::io::Error>
        where I:Iterator<Item=Vec<T>>, T:Encodable {
        let mut n = 0u32;
        let mut wtr = csv::Writer::from_memory();
        for row in rows {
            wtr.encode(row);
            n += 1;
        }

        println!("{}", wtr.as_string());
        Ok(n)
    }
}

//-- CSVWriter
pub struct CSVWriter {
    filepath: String
}

impl CSVWriter {
    pub fn new(the_path: String) -> CSVWriter {
        CSVWriter {filepath: the_path }
    }
}
impl IOWriter for CSVWriter {

    fn write_rows<I, T>(&self, rows: I)  -> Result<u32, std::io::Error>
        where I:Iterator<Item=Vec<T>>, T:Encodable {

        let mut n = 0u32;
        let mut wtr = csv::Writer::from_file(&self.filepath).expect("Failed to open output file");
        wtr.flush();

        for row in rows {
            wtr.encode(row);
            n += 1;
        }

        Ok(n)
    }
}


