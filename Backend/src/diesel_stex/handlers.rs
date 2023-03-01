use diesel::{pg::*, prelude::*};

use super::models::*;
use crate::actix_stex::models::{AccountID, NewUser};

// Autocorrect
pub fn get_all_dnames(db: &mut PgConnection, prefix: &str) -> Vec<User> {
    use crate::schema::users::{display_name, dsl, id};
    dsl::users
        .select((id, display_name))
        .filter(display_name.ilike(format!("{prefix}%")))
        .limit(20)
        .load::<User>(db)
        .unwrap()
}

pub fn get_all_pnames(db: &mut PgConnection, prefix: &str) -> Vec<APIResult> {
    use crate::schema::posts::{dsl, id, title};
    dsl::posts
        .select((id, title.assume_not_null()))
        .filter(title.is_not_null())
        .filter(title.ilike(format!("{prefix}%")))
        .limit(20)
        .load::<APIResult>(db)
        .unwrap()
}

pub fn get_all_tagnames(db: &mut PgConnection, prefix: &str) -> Vec<APIResult> {
    use crate::schema::tags::{dsl, id, tag_name};
    dsl::tags
        .select((id, tag_name))
        .filter(tag_name.ilike(format!("{prefix}%")))
        .limit(20)
        .load::<APIResult>(db)
        .unwrap()
}

// Search
pub fn question_search_title(
    db: &mut PgConnection,
    req: &str,
    ord: &str,
    ad: bool,
) -> Vec<DisplayPost> {
    use crate::schema::posts::{dsl::posts, *};
    let q = posts
        .filter(title.ilike(format!("%{req}%")))
        .filter(parent_id.is_null());
    if ord == "score" {
        if ad {
            q.order(score).get_results::<DisplayPost>(db).unwrap()
        } else {
            q.order(score.desc())
                .get_results::<DisplayPost>(db)
                .unwrap()
        }
    } else {
        if ad {
            q.order(creation_date)
                .get_results::<DisplayPost>(db)
                .unwrap()
        } else {
            q.order(creation_date.desc())
                .get_results::<DisplayPost>(db)
                .unwrap()
        }
    }
}

pub fn question_search_owner(
    db: &mut PgConnection,
    req: &i32,
    ord: &str,
    ad: bool,
) -> Vec<DisplayPost> {
    use crate::schema::posts::{dsl::posts, *};
    let q = posts
        .filter(owner_user_id.eq(req))
        .filter(parent_id.is_null());
    if ord == "score" {
        if ad {
            q.order(score).get_results::<DisplayPost>(db).unwrap()
        } else {
            q.order(score.desc())
                .get_results::<DisplayPost>(db)
                .unwrap()
        }
    } else {
        if ad {
            q.order(creation_date)
                .get_results::<DisplayPost>(db)
                .unwrap()
        } else {
            q.order(creation_date.desc())
                .get_results::<DisplayPost>(db)
                .unwrap()
        }
    }
}

pub fn answer_search_owner(
    db: &mut PgConnection,
    req: &i32,
    ord: &str,
    ad: bool,
) -> Vec<DisplayPost> {
    use crate::schema::posts::{dsl::posts, *};
    let q = posts
        .filter(owner_user_id.eq(req))
        .filter(parent_id.is_not_null());
    if ord == "score" {
        if ad {
            q.order(score).get_results::<DisplayPost>(db).unwrap()
        } else {
            q.order(score.desc())
                .get_results::<DisplayPost>(db)
                .unwrap()
        }
    } else {
        if ad {
            q.order(creation_date)
                .get_results::<DisplayPost>(db)
                .unwrap()
        } else {
            q.order(creation_date.desc())
                .get_results::<DisplayPost>(db)
                .unwrap()
        }
    }
}

pub fn post_search_tags(db: &mut PgConnection, req: &str, ord: &str, ad: bool) -> Vec<DisplayPost> {
    use crate::schema::posts::{dsl::posts, *};
    let q = posts
        .filter(tags.ilike(format!("%{req}%")))
        .filter(parent_id.is_null());

    if ord == "score" {
        if ad {
            q.order(score).get_results::<DisplayPost>(db).unwrap()
        } else {
            q.order(score.desc())
                .get_results::<DisplayPost>(db)
                .unwrap()
        }
    } else {
        if ad {
            q.order(creation_date)
                .get_results::<DisplayPost>(db)
                .unwrap()
        } else {
            q.order(creation_date.desc())
                .get_results::<DisplayPost>(db)
                .unwrap()
        }
    }
}

pub fn post_search_many_tags(
    db: &mut PgConnection,
    req: &str,
    ord: &str,
    ad: bool,
) -> Vec<DisplayPost> {
    let mut v: Vec<String> = req
        .chars()
        .filter(|u| !u.is_whitespace())
        .collect::<String>()
        .split("<")
        .map(|s| String::from("<") + s)
        .collect();
    v.remove(0);

    let inds = v
        .iter()
        .map(|s| post_search_tags(db, s, ord, ad))
        .collect::<Vec<Vec<DisplayPost>>>();
    let out = &mut inds[0]
        .iter()
        .map(|u| u.clone())
        .collect::<Vec<DisplayPost>>();
    for q in inds.iter() {
        out.retain(|x| q.contains(x));
    }
    // out[..20].to_vec()
    out.truncate(20);
    out.clone()
}

pub fn uid_unm(db: &mut PgConnection, uid: &i32) -> String {
    use crate::schema::users::dsl::*;
    users.filter(id.eq(uid)).select(display_name).get_result::<String>(db).unwrap()
}
// User stuff
pub fn new_post(
    db: &mut PgConnection,
    new: &NewPost,
    oid: &i32,
) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    // new.creation_date = chrono::offset::Local::now().naive_utc();
    diesel::insert_into(posts)
        .values((
            &*new,
            owner_user_id.eq(oid),
            id.eq(&get_next_pid(db)),
            owner_display_name.eq(uid_unm(db, oid)),
            creation_date.eq(chrono::offset::Local::now().naive_utc()),
        ))
        .get_result(db)
}

pub fn answer(
    db: &mut PgConnection,
    new: &AnswerPost,
    oid: &i32,
    par_id: &i32,
) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    diesel::insert_into(posts)
        .values((
            new,
            owner_user_id.eq(oid),
            id.eq(&get_next_pid(db)),
            parent_id.eq(par_id),
            owner_display_name.eq(uid_unm(db, oid)),
            creation_date.eq(chrono::offset::Local::now().naive_utc()),
        ))
        .get_result(db)
}

pub fn update(
    db: &mut PgConnection,
    new: &OldPost,
    it: &i32,
    me: &i32,
) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    diesel::update(posts.filter(owner_user_id.eq(me)).filter(id.eq(it)))
        .set((tags.eq(&new.tags), body.eq(&new.body), title.eq(&new.title)))
        .get_result(db)
}

pub fn delete(
    db: &mut PgConnection,
    kill: &i32,
    me: &i32,
) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    diesel::delete(posts.filter(owner_user_id.eq(me)).filter(id.eq(kill))).get_result(db)
}

pub fn all_answers(
    db: &mut PgConnection,
    parent: &i32,
) -> Result<Vec<DisplayPost>, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    posts
        .filter(parent_id.eq(parent))
        .get_results::<DisplayPost>(db)
}

pub fn get_post_by_id(
    db: &mut PgConnection,
    idd: &i32,
) -> Result<DisplayPost, diesel::result::Error> {
    use crate::schema::posts::dsl::*;
    posts.filter(id.eq(idd)).get_result::<DisplayPost>(db)
}

pub fn iam(db: &mut PgConnection, idd: &i32) -> Result<DisplayUser, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    users.filter(id.eq(idd)).get_result::<DisplayUser>(db)
}

// serial type generators
fn get_next_uid(db: &mut PgConnection) -> i32 {
    use crate::schema::users::dsl::*;
    users
        .select(id)
        .order(id.desc())
        .limit(1)
        .get_result::<i32>(db)
        .unwrap()
        + 1
}

fn get_next_pid(db: &mut PgConnection) -> i32 {
    use crate::schema::posts::dsl::*;
    posts
        .select(id)
        .order(id.desc())
        .limit(1)
        .get_result::<i32>(db)
        .unwrap()
        + 1
}

pub fn makeme(db: &mut PgConnection, body: NewUser) -> Result<AccountID, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;
    use crate::schema::users::dsl::*;
    // use super::models::UsersPKey;
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.hash.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let new = UsersPKey {
        id: get_next_uid(db),
        display_name: body.display_name.clone(),
        creation_date: body.crnd,
        last_access_date: body.crnd,
    };
    let res1 = diesel::insert_into(users)
        .values(new)
        .get_result::<DisplayUser>(db)?;

    diesel::insert_into(accounts)
        .values((
            crate::schema::accounts::dsl::id.eq(res1.id),
            username.eq(&body.display_name),
            password.eq(hashed_password),
        ))
        .get_result::<AccountID>(db)
}

pub fn acc_by_id(db: &mut PgConnection, idd: &i32) -> Result<AccountID, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;
    accounts.filter(id.eq(idd)).get_result::<AccountID>(db)
}

pub fn acc_by_unm(db: &mut PgConnection, idd: &str) -> Result<AccountID, diesel::result::Error> {
    use crate::schema::accounts::dsl::*;
    accounts
        .filter(username.eq(idd))
        .get_result::<AccountID>(db)
}

pub fn dupe_acc(db: &mut PgConnection, unm: &str) -> bool {
    use crate::schema::accounts::dsl::*;
    !accounts
        .filter(username.eq(unm))
        .get_results::<AccountID>(db)
        .unwrap()
        .is_empty()
}

pub fn make_bio(
    db: &mut PgConnection,
    bio: &str,
    idd: &i32,
) -> Result<DisplayUser, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .filter(id.eq(idd))
        .set(about_me.eq(bio))
        .get_result(db)
}
