#![allow(unused_assignments)]
use std::io::{stdin, Read};

use DBMS::*;
use text_io::scan;

// #[cfg(not(windows))]
// const EOF: &str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &str = "CTRL+Z";

fn main() {
    let db = &mut connect();
    let mut title = String::new();
    let mut body = String::new();

    println!("Title: ");
    scan!("{}", title);

    println!("Body: ");
    stdin().read_to_string(&mut body).unwrap();

    let title = title.trim();
    let body = body.trim();
    let post = push(db, title, body);

    println!("Saved: {} {title}", post.id);
}