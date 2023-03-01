-- Your SQL goes here
update posts set owner_display_name = (select display_name from users where users.id = posts.owner_user_id);