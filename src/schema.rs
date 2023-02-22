// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}

diesel::table! {
    badges (id) {
        id -> Int4,
        user_id -> Int4,
        class -> Int2,
        name -> Varchar,
        tag_based -> Bool,
        date -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Nullable<Int4>,
        score -> Int2,
        content_license -> Varchar,
        user_display_name -> Nullable<Varchar>,
        text -> Nullable<Text>,
        creation_date -> Timestamp,
    }
}

diesel::table! {
    dummys (id) {
        id -> Int4,
        a -> Nullable<Int4>,
        b -> Nullable<Int4>,
    }
}

diesel::table! {
    post_history (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Nullable<Int4>,
        post_history_type_id -> Int2,
        user_display_name -> Nullable<Varchar>,
        content_license -> Nullable<Varchar>,
        revision_guid -> Nullable<Uuid>,
        text -> Nullable<Text>,
        comment -> Nullable<Text>,
        creation_date -> Timestamp,
    }
}

diesel::table! {
    post_links (id) {
        id -> Int4,
        related_post_id -> Int4,
        post_id -> Int4,
        link_type_id -> Int2,
        creation_date -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        owner_user_id -> Nullable<Int4>,
        last_editor_user_id -> Nullable<Int4>,
        post_type_id -> Int2,
        accepted_answer_id -> Nullable<Int4>,
        score -> Int4,
        parent_id -> Nullable<Int4>,
        view_count -> Nullable<Int4>,
        answer_count -> Nullable<Int4>,
        comment_count -> Nullable<Int4>,
        owner_display_name -> Nullable<Varchar>,
        last_editor_display_name -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        tags -> Nullable<Varchar>,
        content_license -> Varchar,
        body -> Nullable<Text>,
        favorite_count -> Nullable<Int4>,
        creation_date -> Timestamp,
        community_owned_date -> Nullable<Timestamp>,
        closed_date -> Nullable<Timestamp>,
        last_edit_date -> Nullable<Timestamp>,
        last_activity_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        excerpt_post_id -> Nullable<Int4>,
        wiki_post_id -> Nullable<Int4>,
        tag_name -> Varchar,
        count -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        account_id -> Nullable<Int4>,
        reputation -> Int4,
        views -> Nullable<Int4>,
        down_votes -> Nullable<Int4>,
        up_votes -> Nullable<Int4>,
        display_name -> Varchar,
        location -> Nullable<Varchar>,
        profile_image_url -> Nullable<Varchar>,
        website_url -> Nullable<Varchar>,
        about_me -> Nullable<Text>,
        creation_date -> Timestamp,
        last_access_date -> Timestamp,
    }
}

diesel::table! {
    votes (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        post_id -> Int4,
        vote_type_id -> Int2,
        bounty_amount -> Nullable<Int2>,
        creation_date -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    badges,
    comments,
    dummys,
    post_history,
    post_links,
    posts,
    tags,
    users,
    votes,
);
