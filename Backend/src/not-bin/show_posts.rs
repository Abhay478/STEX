#![allow(unused_assignments)]
use DBMS::models::*;
use diesel::prelude::*;
use DBMS::*;
use text_io::scan;

fn main() {
    use DBMS::diesel_stex::schema::posts::dsl::*;
    let db = &mut connect();
    println!("Which?");
    let mut resp = String::new();
    scan!("{}",resp);
    let resp = resp.trim();
    let out = match resp {
        "published" => posts.filter(published.eq(true)).limit(5).load::<Post>(db).unwrap(),
        "draft" => posts.filter(published.eq(false)).limit(5).load::<Post>(db).unwrap(),
        _ => posts.limit(5).load::<Post>(db).unwrap(),
    };
    for post in out {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}