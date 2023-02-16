use diesel::prelude::*;
use crate::{schema::*, diesel_stex::models::DisplayPost};
use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Clone)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub username: Option<String>,
    pub password_hash: Option<String>,
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
    pub a: Vec<DisplayPost>
}