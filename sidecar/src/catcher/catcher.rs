use core::time;
use std::{
    fmt::Error,
    fs::{self, File},
    io::{Read, Write},
    os::unix::net::UnixListener,
    thread::{self, JoinHandle},
};

use bincode::deserialize;
use serde::{Deserialize, Serialize};

use crate::log::log::Log;

pub trait Catcher {
    fn start(self) -> JoinHandle<String>;
}

pub struct FileBasedCatcher {}

pub struct SocketBasedCatcher {}

impl Catcher for FileBasedCatcher {
    fn start(self) -> JoinHandle<String> {
        thread::spawn(|| {
            loop {
                let log_dir_path = String::from("/home/raonismaneoto/workspace/rlogger");

                let maybe_logs = fs::read_dir(log_dir_path);
                match maybe_logs {
                    Ok(paths) => {
                        for path in paths {
                            let maybe_content =
                                read_and_parse_file_content(path.unwrap().path().to_str().unwrap());
                            match maybe_content {
                                Ok(content) => {
                                    print!("Content msg {}", content.message);
                                }
                                Err(err) => {
                                    print!("Unable to read log content");
                                }
                            }
                        }
                    }
                    Err(err) => {
                        // send an error log to the log server?
                        return String::from("not able to read logs dir");
                    }
                }

                thread::sleep(time::Duration::from_secs(30));
            }
        })
    }
}

impl Catcher for SocketBasedCatcher {
    fn start(self) -> JoinHandle<String> {
        thread::spawn(|| {
            let listener = UnixListener::bind("/tmp/rst.sock").unwrap();
            println!(
                "listening on port {}",
                listener
                    .local_addr()
                    .unwrap()
                    .as_pathname()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
            std::io::stdout().flush().unwrap();

            loop {
                match listener.accept() {
                    Ok((socket, _)) => {
                        let mut socket_copy = socket.try_clone().unwrap();
                        thread::spawn(move || {
                            let mut data = Vec::new();
                            socket_copy.read_to_end(&mut data);

                            let maybe_log = serde_json::from_slice::<Log>(&data);

                            match maybe_log {
                                Ok(log) => {
                                    println!("{}", log);
                                    std::io::stdout().flush().unwrap();
                                }
                                Err(err) => {
                                    println!(
                                        "unable to deserialize received log. {}",
                                        err.to_string()
                                    );
                                    std::io::stdout().flush().unwrap();
                                }
                            }
                        });
                    }
                    Err(e) => return String::from(format!("accept function failed: {:?}", e)),
                }
            }
        })
    }
}

fn read_and_parse_file_content(file_path: &str) -> Result<Log, Error> {
    let mut file = File::open(file_path).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let result: Log = serde_json::from_str(&buff).unwrap();

    Ok(result)
}
