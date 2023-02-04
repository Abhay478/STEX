use chrono::NaiveDateTime;
use diesel::{prelude::*, sql_types::Timestamp};
use serde_derive::Deserialize;
use crate::schema::*;

// #[derive(Queryable, Debug)]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }

#[derive(Insertable, Deserialize)]
#[diesel(table_name = dummys)]
pub struct Dummy {
    pub a: i32,
    pub b: i32
}

#[derive(Queryable)]
pub struct DummyRes {
    pub id: i32,
    pub a: Option<i32>,
    pub b: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    display_name: String,
    creation_date: NaiveDateTime,
    reputation: i32,
}

// #[derive(Insertable)]
// #[diesel(table_name = posts)]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }