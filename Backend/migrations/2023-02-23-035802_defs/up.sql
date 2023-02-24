-- Your SQL goes here
alter table posts alter column post_type_id set default 0;
alter table posts alter column score set default 0;
alter table posts alter column content_license set default 'None';