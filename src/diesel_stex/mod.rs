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
use actix_web::Responder;
use diesel::pg::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use std::env;
use diesel::query_dsl::*;

use crate::Pool;
use crate::schema;


use self::models::Dummy;
use self::models::DummyRes;
use self::models::DisplayPost;
// use self::models::NewPost;
// use self::models::Post;

pub mod models;
pub fn connect() -> Pool {
    dotenv().ok();

    let url = env::var("DATABASE_URL").unwrap();
    // PgConnection::establish(&url).unwrap_or_else(|_| panic!("Oops."))
    let cn = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder().build(cn).unwrap()
}


// pub fn push(db: &mut PgConnection, title: &str, body: &str) -> Post {
//     use crate::schema::posts;
//     let new = NewPost {title, body};

//     diesel::insert_into(posts::table).values(&new).get_result(db).unwrap()
// }

// pub fn publish(db: &mut PgConnection, title: &str) -> Post {
//     use self::schema::posts;
//     diesel::update(posts::table.filter(posts::title.eq(title))).set(posts::published.eq(true)).get_result(db).unwrap()
// }

pub fn push(db: &mut PgConnection) -> DummyRes {
    let new = Dummy {a: 1, b: 2};
    diesel::insert_into(crate::schema::dummys::table).values(&new).get_result(db).unwrap()
}

pub fn accept(db: &mut PgConnection, req: (i32, i32)) -> DummyRes {
    let new = Dummy {a: req.0, b: req.1};
    diesel::insert_into(crate::schema::dummys::table).values(&new).get_result(db).unwrap()
}

pub fn accept_struct(db: &mut PgConnection, req: Dummy) -> DummyRes {
    diesel::insert_into(crate::schema::dummys::table).values(&req).get_result(db).unwrap()
}

pub fn get_all_dnames(db: &mut PgConnection) -> Vec<String> {
    use crate::schema::users::display_name;
    schema::users::dsl::users.select(display_name).load::<String>(db).unwrap()
}

pub fn get_all_tagnames(db: &mut PgConnection) -> Vec<String> {
    use crate::schema::tags::tag_name;
    schema::tags::dsl::tags.select(tag_name).load::<String>(db).unwrap()
}

enum PSC {
    UID(usize),
    UTG(String),
    VTG(Vec<String>)
}

// pub fn post_search(db: &mut PgConnection, crit: PSC) -> Vec<DisplayPost> {
//     match crit {
//         PSC::UID(x) => {schema::posts::dsl::posts.filter(schema::posts::owner_user_id.eq(Some(x))).get_results::<DisplayPost>(db).unwrap()},
//         PSC::UTG(t) => 
//     }
// }