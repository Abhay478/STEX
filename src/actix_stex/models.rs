use crate::{diesel_stex::models::DisplayPost, schema::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Clone)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Clone)]
#[diesel(table_name = accounts)]
pub struct AccountID {
    pub id: i32,
    pub username: Option<String>,
    pub password_hash: Option<String>,
}

// #[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
// #[diesel(table_name = users)]
// pub struct User {

// }

#[derive(Deserialize, Serialize)]
pub struct Page {
    pub q: DisplayPost,
    pub a: Vec<DisplayPost>,
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub display_name: String,
    pub hash: String,
    pub crnd: NaiveDateTime,
}
