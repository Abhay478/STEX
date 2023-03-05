use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Insertable, Deserialize)]
#[diesel(table_name = dummys)]
pub struct Dummy {
    pub a: i32,
    pub b: i32,
}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct Params {
    pub q: String,
}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct ParamsInt {
    pub q: i32,
}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct ParamsTwo {
    pub attr: String, //try and make it &str later // Can't, coz weird lifetime errors.
    pub dir: bool,
}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct ParamsAll {
    pub attr: String, //try and make it &str later // Can't, coz weird lifetime errors.
    pub dir: bool,
    pub text: String,
}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct APIResult {
    pub id: i32,
    pub text: String,
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
    pub id: i32,
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
//     pub id: i32,
//     pub tags: Vec<String>,
// }

#[derive(Queryable, Serialize, Deserialize, PartialEq, Clone)]
#[diesel(table_name = posts)]
pub struct DisplayPost {
    pub id: i32,
    pub owner_user_id: Option<i32>,
    pub last_editor_user_id: Option<i32>,
    pub post_type_id: i16,
    pub accepted_answer_id: Option<i32>,
    pub score: i32,
    pub parent_id: Option<i32>,
    pub view_count: Option<i32>,
    pub answer_count: Option<i32>,
    pub comment_count: Option<i32>,
    pub owner_display_name: Option<String>,
    pub last_editor_display_name: Option<String>,
    pub title: Option<String>,
    pub tags: Option<String>,
    pub content_license: String,
    pub body: Option<String>,
    pub favorite_count: Option<i32>,
    pub creation_date: NaiveDateTime,
    pub community_owned_date: Option<NaiveDateTime>,
    pub closed_date: Option<NaiveDateTime>,
    pub last_edit_date: Option<NaiveDateTime>,
    pub last_activity_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Queryable, Serialize, Debug)]
#[diesel(table_name = users)]
pub struct DisplayUser {
    pub id: i32,                           // serial PRIMARY KEY,
    pub account_id: Option<i32>,           //INTEGER,
    pub reputation: i32,                   // INTEGER NOT NULL default 0,
    pub views: Option<i32>,                // INTEGER DEFAULT 0,
    pub down_votes: Option<i32>,           //INTEGER DEFAULT 0,
    pub up_votes: Option<i32>,             //INTEGER DEFAULT 0,
    pub display_name: String,              // VARCHAR(255) NOT NULL default 'Anonymous',
    pub location: Option<String>,          //VARCHAR(512),
    pub profile_image_url: Option<String>, //VARCHAR(255),
    pub website_url: Option<String>,       //VARCHAR(255),
    pub about_me: Option<String>,          //,
    pub creation_date: NaiveDateTime,      // TIMESTAMP NOT NULL,
    pub last_access_date: NaiveDateTime,   // TIMESTAMP NOT NULL
}

#[derive(Insertable, Deserialize, Queryable, Serialize, Debug)]
#[diesel(table_name = users)]
pub struct UsersPKey {
    pub id: i32,
    pub display_name: String,
    pub creation_date: NaiveDateTime,
    pub last_access_date: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    // pub owner_user_id: i32,
    pub title: String,
    pub tags: String,
    pub body: String,
    // pub creation_date: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct OldPost {
    // pub id: i32,
    pub title: String,
    pub tags: String,
    pub body: String,
}

// #[derive(Insertable)]
// #[diesel(table_name = posts)]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }
#[derive(Insertable, Deserialize, Queryable, Serialize, Debug)]
#[diesel(table_name = votes)]
pub struct DisplayVote {
    pub id: i32,
    pub user_id: Option<i32>,
    pub post_id: i32,
    pub vote_type_id: i16,
    pub bounty_amount: Option<i16>,
    pub creation_date: NaiveDateTime,
}
