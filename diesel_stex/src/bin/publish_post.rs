// use std::io::{stdin, Read};
#![allow(unused_assignments)]
use DBMS::*;
use text_io::scan;

// #[cfg(not(windows))]
// const EOF: &str = "CTRL+D";

// #[cfg(windows)]
// const EOF: &str = "CTRL+Z";

fn main() {
    let db = &mut connect();
    let mut title = String::new();

    println!("Title: ");
    scan!("{}", title);

    let title = title.trim();
    let post = publish(db, title);

    println!("Saved: {} {title}", post.id);
}