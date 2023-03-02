use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, Query},
    HttpResponse, Responder,
};

use crate::{
    auth_stex::{jwt_auth::JwtMiddleware, models::State},
    diesel_stex::{handlers::*, models::*},
};

/// Autocomplete for users: Provide query thus: "/auto/u?q=prefix"
/// Res:
/// [
///    {
///        "id": 42166,
///        "display_name": "abc"
///    },
///    {
///        "id": 56480,
///        "display_name": "abcd"
///    },
///    {
///        "id": 86837,
///        "display_name": "ABCD"
///    }
/// ]
#[get("/auto/u")]
pub async fn get_names(
    state: Data<State>,
    prefix: Query<Params>,
    // _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let all = get_all_dnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

/// Autocomplete for tags: Provide query thus: "/auto/t?q=prefix"
/// Res:
/// [
///     {
///        "id": 53,
///        "text": "actor-model"
///    },
///    {
///        "id": 65,
///        "text": "agile"
///    },
///    {
///        "id": 82,
///        "text": "applications"
///    }
/// ]
#[get("/auto/t")]
pub async fn get_tags(
    state: Data<State>,
    prefix: Query<Params>,
    // _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let all = get_all_tagnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

/// Autocomplete for posts: Provide query thus: "/auto/p?q=prefix"
/// Res:
/// [
///        {
///            "id": 44,
///            "text": "Are certifications worth it?"
///        },
///        {
///            "id": 135,
///            "text": "As a software engineer, who should I be following on Twitter?"
///        },
///        {
///            "id": 206,
///            "text": "Are there areas where TDD provides a high ROI and other areas where the ROI is so low that it is not worth following?"
///        }
/// ]
#[get("/auto/p")]
pub async fn get_qa(
    state: Data<State>,
    prefix: Query<Params>,
    // _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let all = get_all_pnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

/// Post data dump: Provide query thus: "/search/title?attr=["score"|"time"]&dir=[true|false]&text=title"
/// Res:
/// [
/// {
///     "id": 442655,
///     "owner_user_id": 423932,
///     "last_editor_user_id": null,
///     "post_type_id": 0,
///     "accepted_answer_id": null,
///     "score": 0,
///     "parent_id": 442653,
///     "view_count": null,
///     "answer_count": 0,
///     "comment_count": 0,
///     "owner_display_name": null,
///     "last_editor_display_name": null,
///     "title": "Meh1",
///     "tags": "<meh><answer>",
///     "content_license": "None",
///     "body": "Meh2",
///     "favorite_count": null,
///     "creation_date": "2023-02-23T07:52:55.502499",
///     "community_owned_date": null,
///    "closed_date": null,
///     "last_edit_date": null,
///     "last_activity_date": null
/// },
/// {
///     "id": 180531,
///     "owner_user_id": 33410,
///     "last_editor_user_id": null,
///     "post_type_id": 1,
///     "accepted_answer_id": 180533,
///     "score": 0,
///     "parent_id": null,
///     "view_count": 304,
///     "answer_count": 1,
///     "comment_count": 3,
///     "owner_display_name": null,
///     "last_editor_display_name": null,
///     "title": "Did \"Viaweb\" work in the browser without JavaScript and somehow use only Lisp?",
///     "tags": "<programming-languages><history>",
///     "content_license": "CC BY-SA 3.0",
///     "body": "<p>I just read <a href=\"http://www.paulgraham.com/avg.html\" rel=\"nofollow\">Beating the Averages</a>, and Mr. Graham writes that they had a significant advantage over competitors because they used Lisp.</p>\n\n<p>From what I understand, Viaweb was a WYSIWYG editor that ran in the browser for customers to create their own 'stores'. This is obviously inconceivable now to do without JavaScript, yet there is no talk of it at all on this article. Mr. Graham only talks about Lisp and nothing else.</p>\n\n<p>So is (was?) it somehow possible to bypass JavaScript and use Lisp for the front and back ends?</p>\n",
///     "favorite_count": null,
///     "creation_date": "2012-12-24T12:11:15.247",
///     "community_owned_date": null,
///     "closed_date": null,
///     "last_edit_date": null,
///     "last_activity_date": "2012-12-24T13:28:01.150"
/// }
/// ]
#[get("/search/title")]
pub async fn get_question_by_title(
    state: Data<State>,
    order: Query<ParamsAll>,
    // _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = question_search_title(&mut db.get().unwrap(), &order.text, &order.attr, order.dir);
    HttpResponse::Ok().json(post)
}

/// Convention: /user/{user_id}/questions?attr=["score"|"time"]&dir=[true|false]
/// Res:
/// [
///    {
///        "id": 67160,
///        "owner_user_id": -1,
///        "last_editor_user_id": -1,
///        "post_type_id": 5,
///        "accepted_answer_id": null,
///        "score": 0,
///        "parent_id": null,
///        "view_count": null,
///        "answer_count": null,
///        "comment_count": 0,
///        "owner_display_name": null,
///        "last_editor_display_name": null,
///        "title": null,
///        "tags": null,
///        "content_license": "CC BY-SA 3.0",
///        "body": null,
///        "favorite_count": null,
///        "creation_date": "2011-04-11T13:35:04.670",
///        "community_owned_date": null,
///        "closed_date": null,
///        "last_edit_date": "2011-04-11T13:35:04.670",
///        "last_activity_date": "2011-04-11T13:35:04.670"
///    },
///    {
///        "id": 1462,
///        "owner_user_id": -1,
///        "last_editor_user_id": 25936,
///        "post_type_id": 7,
///        "accepted_answer_id": null,
///        "score": 0,
///        "parent_id": null,
///        "view_count": null,
///        "answer_count": null,
///        "comment_count": 0,
///        "owner_display_name": null,
///        "last_editor_display_name": null,
///        "title": null,
///        "tags": null,
///        "content_license": "CC BY-SA 3.0",
///        "body": "<p>Programmers — Stack Exchange is a site for professional programmers who are interested in getting expert answers on conceptual questions about software development. If you have a question about...</p>\n\n<ul>\n<li>algorithm and data structure concepts</li>\n<li>design patterns</li>\n<li>developer testing</li>\n<li>development methodologies</li>\n<li>freelancing and business concerns</li>\n<li>quality assurance</li>\n<li>software architecture</li>\n<li>software engineering</li>\n<li>software licensing</li>\n</ul>\n\n<p>and it is <strong>not about</strong>...</p>\n\n<ul>\n<li>general workplace issues, office politics, résumé help (check out <a href=\"http://workplace.stackexchange.com/\">The Workplace</a> instead),</li>\n<li>implementation issues or programming tools (ask on <a href=\"http://www.stackoverflow.com/\">Stack Overflow</a> instead),</li>\n<li>what language/technology you should learn next, including <a href=\"http://blog.stackoverflow.com/2011/08/gorilla-vs-shark/\">which technology is better</a>,</li>\n<li>what project you should do next,</li>\n<li>what book you should read next,</li>\n<li><a href=\"http://meta.programmers.stackexchange.com/questions/588/are-career-advice-questions-useful-to-anyone-except-the-poster/590#590\">career advice</a>, salary or compensation,</li>\n<li>personal lifestyle, including relationships, and non-programming activities</li>\n</ul>\n\n<p>...then you're in the right place to ask your question!</p>\n\n<p>Please make sure your question uniquely applies to programmers in general:</p>\n\n<p><img src=\"https://i.stack.imgur.com/ociNc.png\" alt=\"proper scope for question\"></p>\n\n<h2>What about subjective questions?</h2>\n\n<p>Subjective questions are allowed, but subjective does not mean &ldquo;anything goes&rdquo;. <strong>Please keep it professional at all times</strong>. If this is a question you'd be uncomfortable discussing with your colleagues in a work environment, it's probably not appropriate here, either.</p>\n\n<p>All subjective questions are expected to be <em>constructive</em>. How do we define that?  Constructive subjective questions &hellip;</p>\n\n<ul>\n<li>inspire answers that explain “why” and “how”.</li>\n<li>tend to have long, not short, answers.</li>\n<li>have a constructive, fair, and impartial tone.</li>\n<li>invite sharing experiences over opinions.</li>\n<li>insist that opinion be backed up with facts and references.</li>\n<li>are more than just mindless social fun.</li>\n</ul>\n\n<p>Questions that do not meet enough of these six guidelines will be closed as \"Not Constructive\". Please see the <a href=\"http://blog.stackoverflow.com/2010/09/good-subjective-bad-subjective\">Good Subjective, Bad Subjective</a> and <a href=\"http://blog.stackoverflow.com/2011/01/real-questions-have-answers/\">Real Questions Have Answers</a> blog posts for more details and examples.</p>\n",
///        "favorite_count": null,
///        "creation_date": "2010-09-08T22:37:51.210",
///        "community_owned_date": "2011-07-05T10:09:01.537",
///        "closed_date": null,
///        "last_edit_date": "2013-03-02T06:31:46.033",
///        "last_activity_date": "2013-03-02T06:31:46.033"
///    }
/// ]
#[get("/user/{id}/questions")]
pub async fn get_questions_by_owner(
    state: Data<State>,
    oid: Path<i32>,
    order: Query<ParamsTwo>,
    // _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = question_search_owner(&mut db.get().unwrap(), &oid, &order.attr, order.dir);
    HttpResponse::Ok().json(post)
}

/// Convention: /user/{user_id}/answers?attr=["score"|"time"]&dir=[true|false]
/// Res:
/// [
///    {
///        "id": 67160,
///        "owner_user_id": -1,
///        "last_editor_user_id": -1,
///        "post_type_id": 5,
///        "accepted_answer_id": null,
///        "score": 0,
///        "parent_id": null,
///        "view_count": null,
///        "answer_count": null,
///        "comment_count": 0,
///        "owner_display_name": null,
///        "last_editor_display_name": null,
///        "title": null,
///        "tags": null,
///        "content_license": "CC BY-SA 3.0",
///        "body": null,
///        "favorite_count": null,
///        "creation_date": "2011-04-11T13:35:04.670",
///        "community_owned_date": null,
///        "closed_date": null,
///        "last_edit_date": "2011-04-11T13:35:04.670",
///        "last_activity_date": "2011-04-11T13:35:04.670"
///    },
///    {
///        "id": 1462,
///        "owner_user_id": -1,
///        "last_editor_user_id": 25936,
///        "post_type_id": 7,
///        "accepted_answer_id": null,
///        "score": 0,
///        "parent_id": null,
///        "view_count": null,
///        "answer_count": null,
///        "comment_count": 0,
///        "owner_display_name": null,
///        "last_editor_display_name": null,
///        "title": null,
///        "tags": null,
///        "content_license": "CC BY-SA 3.0",
///        "body": "<p>Programmers — Stack Exchange is a site for professional programmers who are interested in getting expert answers on conceptual questions about software development. If you have a question about...</p>\n\n<ul>\n<li>algorithm and data structure concepts</li>\n<li>design patterns</li>\n<li>developer testing</li>\n<li>development methodologies</li>\n<li>freelancing and business concerns</li>\n<li>quality assurance</li>\n<li>software architecture</li>\n<li>software engineering</li>\n<li>software licensing</li>\n</ul>\n\n<p>and it is <strong>not about</strong>...</p>\n\n<ul>\n<li>general workplace issues, office politics, résumé help (check out <a href=\"http://workplace.stackexchange.com/\">The Workplace</a> instead),</li>\n<li>implementation issues or programming tools (ask on <a href=\"http://www.stackoverflow.com/\">Stack Overflow</a> instead),</li>\n<li>what language/technology you should learn next, including <a href=\"http://blog.stackoverflow.com/2011/08/gorilla-vs-shark/\">which technology is better</a>,</li>\n<li>what project you should do next,</li>\n<li>what book you should read next,</li>\n<li><a href=\"http://meta.programmers.stackexchange.com/questions/588/are-career-advice-questions-useful-to-anyone-except-the-poster/590#590\">career advice</a>, salary or compensation,</li>\n<li>personal lifestyle, including relationships, and non-programming activities</li>\n</ul>\n\n<p>...then you're in the right place to ask your question!</p>\n\n<p>Please make sure your question uniquely applies to programmers in general:</p>\n\n<p><img src=\"https://i.stack.imgur.com/ociNc.png\" alt=\"proper scope for question\"></p>\n\n<h2>What about subjective questions?</h2>\n\n<p>Subjective questions are allowed, but subjective does not mean &ldquo;anything goes&rdquo;. <strong>Please keep it professional at all times</strong>. If this is a question you'd be uncomfortable discussing with your colleagues in a work environment, it's probably not appropriate here, either.</p>\n\n<p>All subjective questions are expected to be <em>constructive</em>. How do we define that?  Constructive subjective questions &hellip;</p>\n\n<ul>\n<li>inspire answers that explain “why” and “how”.</li>\n<li>tend to have long, not short, answers.</li>\n<li>have a constructive, fair, and impartial tone.</li>\n<li>invite sharing experiences over opinions.</li>\n<li>insist that opinion be backed up with facts and references.</li>\n<li>are more than just mindless social fun.</li>\n</ul>\n\n<p>Questions that do not meet enough of these six guidelines will be closed as \"Not Constructive\". Please see the <a href=\"http://blog.stackoverflow.com/2010/09/good-subjective-bad-subjective\">Good Subjective, Bad Subjective</a> and <a href=\"http://blog.stackoverflow.com/2011/01/real-questions-have-answers/\">Real Questions Have Answers</a> blog posts for more details and examples.</p>\n",
///        "favorite_count": null,
///        "creation_date": "2010-09-08T22:37:51.210",
///        "community_owned_date": "2011-07-05T10:09:01.537",
///        "closed_date": null,
///        "last_edit_date": "2013-03-02T06:31:46.033",
///        "last_activity_date": "2013-03-02T06:31:46.033"
///    }
/// ]
#[get("/user/{id}/answers")]
pub async fn get_answers_by_owner(
    state: Data<State>,
    oid: Path<i32>,
    order: Query<ParamsTwo>,
    // _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = answer_search_owner(&mut db.get().unwrap(), &oid, &order.attr, order.dir);
    HttpResponse::Ok().json(post)
}

/// Post data dump: Provide query thus: "/search/tags?attr=["score"|"time"]&dir=[true|false]&text=<tag_name1><tag_name2>".
/// Looks just like the other two.
#[get("/search/tags")]
pub async fn get_qa_by_tags(
    state: Data<State>,
    order: Query<ParamsAll>,
    // _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = post_search_many_tags(&mut db.get().unwrap(), &order.text, &order.attr, order.dir);
    HttpResponse::Ok().json(post)
}

/// Req:
/// {
/// 	"title": "Meh1",
/// 	"tags": "<meh><answer>",
/// 	"body": "Meh2"
/// }
/// Res: DisplayPost, looks like
/// {
///     "id": 180531,
///     "owner_user_id": 33410,
///     "last_editor_user_id": null,
///     "post_type_id": 1,
///     "accepted_answer_id": 180533,
///     "score": 0,
///     "parent_id": null,
///     "view_count": 304,
///     "answer_count": 1,
///     "comment_count": 3,
///     "owner_display_name": null,
///     "last_editor_display_name": null,
///     "title": "Did \"Viaweb\" work in the browser without JavaScript and somehow use only Lisp?",
///     "tags": "<programming-languages><history>",
///     "content_license": "CC BY-SA 3.0",
///     "body": "<p>I just read <a href=\"http://www.paulgraham.com/avg.html\" rel=\"nofollow\">Beating the Averages</a>, and Mr. Graham writes that they had a significant advantage over competitors because they used Lisp.</p>\n\n<p>From what I understand, Viaweb was a WYSIWYG editor that ran in the browser for customers to create their own 'stores'. This is obviously inconceivable now to do without JavaScript, yet there is no talk of it at all on this article. Mr. Graham only talks about Lisp and nothing else.</p>\n\n<p>So is (was?) it somehow possible to bypass JavaScript and use Lisp for the front and back ends?</p>\n",
///     "favorite_count": null,
///     "creation_date": "2012-12-24T12:11:15.247",
///     "community_owned_date": null,
///     "closed_date": null,
///     "last_edit_date": null,
///     "last_activity_date": "2012-12-24T13:28:01.150"
/// }
#[post("/qa/question")]
pub async fn ask_question(
    state: Data<State>,
    mut new: Json<NewPost>,
    me: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = new_post(&mut db.get().unwrap(), &mut new.0, &me.user_id);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::BadRequest().json(format!(
            "Can't do that: {}.",
            match e {
                diesel::result::Error::AlreadyInTransaction => "Tag error.".to_string(),
                _ => e.to_string(),
            }
        )),
    }
}

/// Req: String body.
/// 
/// Res: DisplayPost, looks like
/// {
///     "id": 180531,
///     "owner_user_id": 33410,
///     "last_editor_user_id": null,
///     "post_type_id": 1,
///     "accepted_answer_id": 180533,
///     "score": 0,
///     "parent_id": null,
///     "view_count": 304,
///     "answer_count": 1,
///     "comment_count": 3,
///     "owner_display_name": null,
///     "last_editor_display_name": null,
///     "title": "Did \"Viaweb\" work in the browser without JavaScript and somehow use only Lisp?",
///     "tags": "<programming-languages><history>",
///     "content_license": "CC BY-SA 3.0",
///     "body": "<p>I just read <a href=\"http://www.paulgraham.com/avg.html\" rel=\"nofollow\">Beating the Averages</a>, and Mr. Graham writes that they had a significant advantage over competitors because they used Lisp.</p>\n\n<p>From what I understand, Viaweb was a WYSIWYG editor that ran in the browser for customers to create their own 'stores'. This is obviously inconceivable now to do without JavaScript, yet there is no talk of it at all on this article. Mr. Graham only talks about Lisp and nothing else.</p>\n\n<p>So is (was?) it somehow possible to bypass JavaScript and use Lisp for the front and back ends?</p>\n",
///     "favorite_count": null,
///     "creation_date": "2012-12-24T12:11:15.247",
///     "community_owned_date": null,
///     "closed_date": null,
///     "last_edit_date": null,
///     "last_activity_date": "2012-12-24T13:28:01.150"
/// }
#[post("/qa/{id}/answer")]
pub async fn give_answer(
    state: Data<State>,
    new: String,
    par: Path<i32>,
    me: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = answer(&mut db.get().unwrap(), &new, &me.user_id, &par);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::Ok().json(format!("Can't do that: {}.", e.to_string())),
    }
}

/// Req:
/// {
///     "title": "Meh-updated",
///     "tags": "<meh><answer><update>",
///     "body": "Meh2 Updated"
/// }
///
/// Res: DisplayPost, looks like
/// {
///     "id": 180531,
///     "owner_user_id": 33410,
///     "last_editor_user_id": null,
///     "post_type_id": 1,
///     "accepted_answer_id": 180533,
///     "score": 0,
///     "parent_id": null,
///     "view_count": 304,
///     "answer_count": 1,
///     "comment_count": 3,
///     "owner_display_name": null,
///     "last_editor_display_name": null,
///     "title": "Did \"Viaweb\" work in the browser without JavaScript and somehow use only Lisp?",
///     "tags": "<programming-languages><history>",
///     "content_license": "CC BY-SA 3.0",
///     "body": "<p>I just read <a href=\"http://www.paulgraham.com/avg.html\" rel=\"nofollow\">Beating the Averages</a>, and Mr. Graham writes that they had a significant advantage over competitors because they used Lisp.</p>\n\n<p>From what I understand, Viaweb was a WYSIWYG editor that ran in the browser for customers to create their own 'stores'. This is obviously inconceivable now to do without JavaScript, yet there is no talk of it at all on this article. Mr. Graham only talks about Lisp and nothing else.</p>\n\n<p>So is (was?) it somehow possible to bypass JavaScript and use Lisp for the front and back ends?</p>\n",
///     "favorite_count": null,
///     "creation_date": "2012-12-24T12:11:15.247",
///     "community_owned_date": null,
///     "closed_date": null,
///     "last_edit_date": null,
///     "last_activity_date": "2012-12-24T13:28:01.150"
/// }
#[post("/qa/{id}/update")]
pub async fn rephrase_qa(
    state: Data<State>,
    new: Json<OldPost>,
    id: Path<i32>,
    me: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = update(&mut db.get().unwrap(), &new.0, &me.user_id, &id);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string())),
    }
}

/// Res: DisplayPost, looks like
/// {
///     "id": 180531,
///     "owner_user_id": 33410,
///     "last_editor_user_id": null,
///     "post_type_id": 1,
///     "accepted_answer_id": 180533,
///     "score": 0,
///     "parent_id": null,
///     "view_count": 304,
///     "answer_count": 1,
///     "comment_count": 3,
///     "owner_display_name": null,
///     "last_editor_display_name": null,
///     "title": "Did \"Viaweb\" work in the browser without JavaScript and somehow use only Lisp?",
///     "tags": "<programming-languages><history>",
///     "content_license": "CC BY-SA 3.0",
///     "body": "<p>I just read <a href=\"http://www.paulgraham.com/avg.html\" rel=\"nofollow\">Beating the Averages</a>, and Mr. Graham writes that they had a significant advantage over competitors because they used Lisp.</p>\n\n<p>From what I understand, Viaweb was a WYSIWYG editor that ran in the browser for customers to create their own 'stores'. This is obviously inconceivable now to do without JavaScript, yet there is no talk of it at all on this article. Mr. Graham only talks about Lisp and nothing else.</p>\n\n<p>So is (was?) it somehow possible to bypass JavaScript and use Lisp for the front and back ends?</p>\n",
///     "favorite_count": null,
///     "creation_date": "2012-12-24T12:11:15.247",
///     "community_owned_date": null,
///     "closed_date": null,
///     "last_edit_date": null,
///     "last_activity_date": "2012-12-24T13:28:01.150"
/// }
#[delete("/qa/{id}/delete")]
pub async fn delete_qa(state: Data<State>, kill: Path<i32>, me: JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let post = delete(&mut db.get().unwrap(), &kill, &me.user_id);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string())),
    }
}

/// Route responds to a get request with struct containing the post corresponding to that id, and all answers to that post.
/// Res:
/// {
/// 	"q": DisplayPost, see above
/// 	"a": list of DisplayPost, see above
/// }
#[get("/qa/{id}")]
pub async fn get_page(
    state: Data<State>,
    id: Path<i32>,
    // _: JwtMiddleware,
) -> impl Responder {
    use crate::actix_stex::models::Page;
    let db = &state.pool;
    let qn = get_post_by_id(&mut db.get().unwrap(), &id);
    match qn {
        Ok(q) => {
            let out = Page {
                q,
                a: all_answers(&mut db.get().unwrap(), &id).unwrap(),
            };
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string())),
    }
}

/// Returns profile
/// Res:
/// {
/// 	"id": 423932,
/// 	"account_id": null,
/// 	"reputation": 0,
/// 	"views": 0,
/// 	"down_votes": 0,
/// 	"up_votes": 0,
/// 	"display_name": "x",
/// 	"location": null,
/// 	"profile_image_url": null,
/// 	"website_url": null,
/// 	"about_me": "abcde",
/// 	"creation_date": "2023-02-23T03:47:24.916123",
/// 	"last_access_date": "2023-02-23T03:47:24.916123"
/// }
#[get("/me")]
pub async fn whoami(state: Data<State>, me: JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let I = iam(&mut db.get().unwrap(), &me.user_id).unwrap();
    HttpResponse::Ok().json(I)
}

/// to update about_me
/// Req: String (Not a json)
/// Res:
/// {
/// 	"id": 423932,
/// 	"account_id": null,
/// 	"reputation": 0,
/// 	"views": 0,
/// 	"down_votes": 0,
/// 	"up_votes": 0,
/// 	"display_name": "x",
/// 	"location": null,
/// 	"profile_image_url": null,
/// 	"website_url": null,
/// 	"about_me": "abcde",
/// 	"creation_date": "2023-02-23T03:47:24.916123",
/// 	"last_access_date": "2023-02-23T03:47:24.916123"
/// }
#[post("/bio")]
pub async fn bio(state: Data<State>, new: String, me: JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let res = make_bio(&mut db.get().unwrap(), &new, &me.user_id);

    HttpResponse::Ok().json(res.unwrap())
}
