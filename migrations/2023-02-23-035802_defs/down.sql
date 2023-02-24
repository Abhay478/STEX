-- This file should undo anything in `up.sql`
alter table posts alter column post_type_id drop default;
alter table posts alter column score drop default;
alter table posts alter column content_license drop default;