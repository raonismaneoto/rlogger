use std::{fmt::Error, fs::{self, File}, thread, time};

use catcher::catcher::{Catcher, SocketBasedCatcher};

pub mod catcher;
pub mod log;

fn main() {
    let catcher = SocketBasedCatcher { };
    
    let worker = catcher.start();

    worker.join().unwrap();
}