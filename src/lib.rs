mod config;
use config::Config;

mod pool;
use pool::Pool;

mod handler;

use std::net::TcpListener;

const HTTP_VERSION: &str = "HTTP/1.1";

fn status_name(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        505 => "HTTP Version Not Supported",
        _ => "",
    }
}

fn status_with_name(status: u16) -> String {
    format!("{} {}", status, status_name(status))
}

pub fn start(config_path: &str) {
    let config = Config::from_file(config_path);
    let address = config.address();
    println!("bind: {}\nroot: {}\n", address, config.root);

    let pool = Pool::init(config);
    let listener = TcpListener::bind(&address).unwrap();
    for stream in listener.incoming() {
        pool.handle(stream.unwrap());
    }
}
