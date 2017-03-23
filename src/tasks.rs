use std::path::PathBuf;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};


fn start_file_scanner(dir_path: &PathBuf) -> (Receiver<>, JoinHandle<()>) {

}