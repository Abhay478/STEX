// use std::collections::HashSet;

// use tokio::spawn;
// use tokio_postgres::{connect, NoTls, Client};


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>>{
//     println!("Hello, world!");
//     let (me, it) = connect("host=localhost dbname=univdb user=MS password=qwerty port=5433", NoTls).await?;
//     spawn(async move {
//         if let Err(e) = it.await {
//             eprintln!("connection error: {}", e);
//         }
//     });

//     let rows = me.query("select sec_id from section where course_id = $1;", &[&"313"]).await?;
//     // dbg!(&rows);
//     for r in rows {
//         for i in 0..r.columns().len() {
//             let v: &str = r.try_get(i).unwrap();
//             println!("{:#?}", v);
//         }
//     }

//     Ok(())
// }

// // fn prereq_wrapper(me: Client) {
// //     println!("Enter course_id.");
// //     let cid: String;
// //     text_io::scan!("{}", cid);
// //     let mut all: HashSet;
// //     prereq_finder(me, cid, &mut all);
// // } 

// // async fn prereq_finder(me: Client, cid: String, all: &mut HashSet<String>) {
// //     let mut firsts = me.query("select prereq_id from prereq where course_id = $1", &[&cid.as_str()]).await.unwrap();

// //     for r in firsts {
// //         all.insert(r.get(0));
// //     }

// // }
#![allow(unused_assignments)]
#![allow(non_snake_case)]
use diesel::pg::*;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::NewPost;
use crate::models::Post;

pub mod models;
pub mod schema;
pub fn connect() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&url).unwrap_or_else(|_| panic!("Oops."))
}

pub fn push(db: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;
    let new = NewPost {title, body};

    diesel::insert_into(posts::table).values(&new).get_result(db).unwrap()
}

pub fn publish(db: &mut PgConnection, title: &str) -> Post {
    use crate::schema::posts;
    diesel::update(posts::table.filter(posts::title.eq(title))).set(posts::published.eq(true)).get_result(db).unwrap()
}