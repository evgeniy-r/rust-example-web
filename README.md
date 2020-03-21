# Simple web file browser

## About

It is a demo project for [Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
of [The Rust Programming Language](https://doc.rust-lang.org/book/) book.

I made it before reading this chapter (and made small changes after), so it differs from one in the book:
* it is much more functional;
* it does not use [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html), only
[channels](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html).   

## Features

* allows to recursively browse files and directories at the host (it automatically generates directory listing
HTML-pages);
* supports many MIMEs (via [mime](https://docs.rs/mime/) and [mime_guess](https://docs.rs/mime_guess/) crates).
* supports GET and HEAD HTTP-methods;
* generates HTML for error pages (codes 400, 404, 500, 501, 505);
* allows configuring via *.toml file ([serde](https://docs.rs/serde/) is used for parsing);
* writes log to stdout.

## Configuration and Using

The web server look for `config.toml` file in its working directory. There is an example configuration with comments in
`config-default.toml` file in the repo. The web server will use defaults if the configuration file is missed. 

Simply build an executable file (with `cargo build --release`), copy it where you want, provide a document root for
browsing with `config.toml` (default is `public` in the web server working directory) and run the executable
file.    