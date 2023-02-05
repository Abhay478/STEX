use chrono::{NaiveDateTime, DateTime, TimeZone, FixedOffset, Utc};
use diesel::{prelude::*, FromSqlRow, AsExpression};
use serde_derive::{Deserialize, Serialize};
use crate::schema::*;
use diesel::sql_types::{Nullable, Timestamp};
// use diesel::deserialize::{self, FromSql};

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

// Autocomplete things
#[derive(Insertable, Deserialize, Queryable, Serialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    // #[diesel(sql_type = Text)]
    pub display_name: String,
}

#[derive(Insertable, Deserialize, Queryable)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub tag_name: String,
}

// #[derive(Queryable, Serialize, Deserialize)]
// #[diesel(table_name = posts)]
// pub struct SearchPost {
//     pub id: usize,
//     pub tags: Vec<String>,
// }

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct DisplayPost {
    pub id: usize,
    pub tags: Vec<String>,
    pub body: Option<String>,
    pub title: Option<String>,
    pub view_count: Option<usize>,
    pub creation_date: DateTime<Utc>,
    pub community_owned_date: Option<DateTime<Utc>>,
    pub closed_date: Option<DateTime<Utc>>,
    pub last_edit_date: Option<DateTime<Utc>>,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub score: usize, 
    pub owner_display_name: Option<String>,
    pub accepted_answer_id: Option<usize>,
    pub last_editor_user_id: Option<usize>,
    pub post_type_id: usize,
    pub parent_id: Option<usize>,
    pub answer_count: Option<usize>,
    pub comment_count: Option<usize>,
    pub last_editor_display_name: Option<String>,
    pub content_license: String,
    pub favorite_count: Option<usize>,
}

// #[derive(Insertable)]
// #[diesel(table_name = posts)]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }