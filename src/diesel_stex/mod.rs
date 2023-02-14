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
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use std::env;

use crate::Pool;
use crate::diesel_stex::models::AutocResults;
use crate::diesel_stex::models::AnswerPost;


// use self::models::AutocParamsAll;
// use self::models::Dummy;
// use self::models::DummyRes;
use self::models::DisplayPost;
use self::models::NewPost;
use self::models::User;
// use self::models::NewPost;
// use self::models::Post;

pub mod models;
pub fn connect() -> Pool {
    dotenv().ok();

    let url = env::var("DATABASE_URL").unwrap();
    // PgConnection::establish(&url).unwrap_or_else(|_| panic!("Oops."))
    let cn = ConnectionManager::<PgConnection>::new(url);
    println!("Connected to database.");
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

// pub fn push(db: &mut PgConnection) -> DummyRes {
//     let new = Dummy {a: 1, b: 2};
//     diesel::insert_into(crate::schema::dummys::table).values(&new).get_result(db).unwrap()
// }

// pub fn accept(db: &mut PgConnection, req: (i32, i32)) -> DummyRes {
//     let new = Dummy {a: req.0, b: req.1};
//     diesel::insert_into(crate::schema::dummys::table).values(&new).get_result(db).unwrap()
// }

// pub fn accept_struct(db: &mut PgConnection, req: Dummy) -> DummyRes {
//     diesel::insert_into(crate::schema::dummys::table).values(&req).get_result(db).unwrap()
// }

pub fn get_all_dnames(db: &mut PgConnection, prefix: &str) -> Vec<User> {
    use crate::schema::users::{display_name, dsl, id};
    dsl::users.select((id, display_name)).filter(display_name.like(format!("%{prefix}%"))).load::<User>(db).unwrap()
}

pub fn get_all_pnames(db: &mut PgConnection, prefix: &str) -> Vec<AutocResults> {
    use crate::schema::posts::{title, dsl, id};
    dsl::posts.select((id, title.assume_not_null())).filter(title.is_not_null()).filter(title.like(format!("%{prefix}%"))).load::<AutocResults>(db).unwrap()
}

pub fn get_all_tagnames(db: &mut PgConnection, prefix: &str) -> Vec<AutocResults> {
    use crate::schema::tags::{tag_name, dsl, id};
    dsl::tags.select((id, tag_name)).filter(tag_name.like(format!("%{prefix}%"))).load::<AutocResults>(db).unwrap()
}

pub fn post_search_title(db: &mut PgConnection, req: &str) -> Vec<DisplayPost>{
    use crate::schema::posts::*;
    dsl::posts.filter(title.eq(req)).get_results::<DisplayPost>(db).unwrap()
}

pub fn post_search_owner(db: &mut PgConnection, req: i32) -> Vec<DisplayPost> {
    use crate::schema::posts::*;
    dsl::posts.filter(owner_user_id.eq(req)).get_results::<DisplayPost>(db).unwrap()
}

pub fn post_search_tags(db: &mut PgConnection, req: &str) -> Vec<DisplayPost> {
    use crate::schema::posts::{*, dsl::posts};
    posts.filter(tags.like(format!("%{req}%"))).get_results::<DisplayPost>(db).unwrap()
}

pub fn post_search_many_tags(db: &mut PgConnection, req: &str) -> Vec<DisplayPost> {
    use crate::schema::posts::{*, dsl::posts};
    posts.filter(tags.similar_to(format!("%{req}%"))).get_results::<DisplayPost>(db).unwrap()
}

// pub fn nuanced_search(db: &mut PgConnection, req: AutocParamsAll) -> Vec<DisplayPost> {
//     use crate::schema::posts::{*, dsl::posts};
//     posts.filter(tags.like(format!("%{}%", req.tag))).filter(owner_user_id.eq(req.uid)).filter(title.like(format!("%{}%", req.title))).get_results::<DisplayPost>(db).unwrap()
// }

pub fn new_post(db: &mut PgConnection, new: &NewPost) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::posts;
    diesel::insert_into(posts).values(new).get_result(db)
}

pub fn answer(db: &mut PgConnection, new: &AnswerPost) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::posts;
    diesel::insert_into(posts).values(new).get_result(db)
}

pub fn update(db: &mut PgConnection, new: &NewPost) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    diesel::update(posts.filter(id.eq(new.id))).set(tags.eq(&new.tags)).get_result(db)
}

pub fn delete(db: &mut PgConnection, kill: &i32) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    diesel::delete(posts.filter(id.eq(kill))).get_result(db)
}

pub fn all_answers(db: &mut PgConnection, parent: &i32) -> Result<Vec<DisplayPost>, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    posts.filter(parent_id.eq(parent)).get_results::<DisplayPost>(db)
}