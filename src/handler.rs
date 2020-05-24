use crate::Config;

mod request;
use request::Request;

mod response;
use response::Response;

mod page;
use page::dir::{self, DirItem};
use page::error;

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub struct Handler<'a> {
    stream: TcpStream,
    config: &'a Config,
    worker_id: usize,
}

impl<'a> Handler<'a> {
    pub fn new(config: &'a Config, stream: TcpStream, worker_id: usize) -> Self {
        stream
            .set_read_timeout(Some(Duration::new(config.read_timeout, 0)))
            .unwrap();
        Self {
            config,
            stream,
            worker_id,
        }
    }

    pub fn handle(self) {
        let worker_id = self.worker_id;
        println!("worker {}: stream", worker_id);
        match self.process_request() {
            Some(size) => println!("worker {}: {} bytes", worker_id, size),
            None => {}
        };
    }

    fn process_request(mut self) -> Option<usize> {
        let mut reader = BufReader::new(&self.stream);
        let mut start_line = String::new();
        reader.read_line(&mut start_line).ok()?;

        let response = self.response(start_line);

        let mut counter = 0;
        counter += self.write(response.start_line().as_bytes())?;
        counter += self.write(response.header_lines().as_bytes())?;
        counter += self.write("\n".as_bytes())?;
        if response.send_body {
            counter += self.write(&response.body)?;
        };
        self.stream.flush().ok()?;

        Some(counter)
    }

    fn write(&mut self, data: &[u8]) -> Option<usize> {
        self.stream.write(data).ok()
    }

    fn filepath(&self, path: &str) -> Option<PathBuf> {
        let mut filepath = self.config.parsed_root();

        let decoded_path = percent_encoding::percent_decode_str(path)
            .decode_utf8()
            .ok()?;
        let path = Path::new(decoded_path.as_ref());
        for c in path.components().skip(1) {
            filepath.push(c);
        }

        let filepath = filepath.canonicalize().ok()?;

        if filepath.starts_with(self.config.parsed_root()) {
            Some(filepath)
        } else {
            None
        }
    }

    fn response(&self, start_line: String) -> Response {
        let start_line = start_line.trim();
        println!("worker {}: {}", self.worker_id, start_line);

        let request = match Request::from(start_line) {
            Some(r) => r,
            None => return self.response_for_error(400, true),
        };
        let send_body = !request.is_head();

        if !request.is_correct_proto() {
            return self.response_for_error(505, send_body);
        }

        if !(request.is_get() || request.is_head()) {
            return self.response_for_error(501, send_body);
        }

        match self.filepath(request.path) {
            Some(p) => {
                if p.is_dir() {
                    match p.strip_prefix(self.config.parsed_root()) {
                        Ok(rel_p) => self.response_for_dir(rel_p, &p, send_body),
                        Err(e) => {
                            println!("worker {}: {}", self.worker_id, e);
                            self.response_for_error(500, send_body)
                        }
                    }
                } else {
                    self.response_for_file(&p, send_body)
                }
            }
            None => self.response_for_error(404, send_body),
        }
    }

    fn response_for_file(&self, p: &PathBuf, send_body: bool) -> Response {
        match fs::read(p) {
            Ok(body) => {
                let mime = mime_guess::from_path(p)
                    .first_raw()
                    .unwrap_or(mime::TEXT_PLAIN.essence_str());
                println!("worker {}: file {}", self.worker_id, p.to_string_lossy());
                Response::new(200, body, send_body, mime)
            }
            Err(e) => {
                println!("{}", e);
                self.response_for_error(500, send_body)
            }
        }
    }

    fn response_for_dir(&self, rel_p: &Path, p: &PathBuf, send_body: bool) -> Response {
        println!(
            "worker {}: directory index {}",
            self.worker_id,
            rel_p.to_string_lossy()
        );
        match fs::read_dir(p) {
            Ok(items) => {
                let paths: Vec<_> = items.filter_map(|i| i.ok().map(|i| i.path())).collect();

                let rel_paths: Vec<_> = paths
                    .iter()
                    .filter_map(|i| DirItem::from_abs_path(i, &self.config.parsed_root()))
                    .collect();

                Response::new(
                    200,
                    dir::body(rel_p, rel_paths),
                    send_body,
                    mime::TEXT_HTML.essence_str(),
                )
            }
            Err(e) => {
                println!("{}", e);
                self.response_for_error(500, send_body)
            }
        }
    }

    fn response_for_error(&self, status: u16, send_body: bool) -> Response {
        println!("worker {}: error {}", self.worker_id, status);
        Response::new(
            status,
            error::body(status),
            send_body,
            mime::TEXT_HTML.essence_str(),
        )
    }
}
