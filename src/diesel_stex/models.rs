use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = dummys)]
pub struct Dummy {
    pub a: i32,
    pub b: i32
}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct AutocParams {
    pub q: String
}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct AutocParamsInt {
    pub q: usize
}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct AutocParamsAll {
    pub tag: String, //try and make it &str later // Can't, coz weird lifetime errors.
    pub uid: i32,
    pub title: String

}

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct AutocResults {
    pub id: i32,
    pub text: String
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

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub owner_user_id: i32,
    pub title: String,
    pub tags: String,
    pub body: String,
    pub creation_date: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct OldPost {
    pub id: i32,
    pub title: String,
    pub tags: String,
    pub body: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct AnswerPost {
    pub owner_user_id: i32,
    pub title: String,
    pub tags: String,
    pub body: String,
    pub parent_id: i32, // the important one
}



// #[derive(Insertable)]
// #[diesel(table_name = posts)]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }