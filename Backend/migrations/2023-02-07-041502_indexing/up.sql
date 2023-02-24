-- Your SQL goes here

create index posts_title on posts(id, title);
create index users_dislay_name on users(id, display_name);
create index tags_tag_name on tags(id, tag_name);