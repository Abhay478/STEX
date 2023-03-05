# DBMS-2 (CS3550) : Assignment 2

## Installation and execution instructions
- Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Flutter: `https://docs.flutter.dev/get-started/install/linux`
  
## Setup
- Backend: 
  - `cd Backend`
  - `cargo install diesel_cli`
  - `diesel setup`
  - `diesel migration run`
  - `cargo r`
- Frontend: 
  - `cd Frontend`
  - `flutter pub get`
  - `flutter run -d web-server --web-port 3000 --web-renderer html`
- Go to `http://localhost:3000`
  
## Summary
- The application is named STEX, an abbreviation of Stack Exchange - which it is based on.
- While not a complete replica of https://stackexchange.com, it contains some of the most important features, including
  - Token-based authentication with Sessions
  - Account creation with secure encryption for passwords.
  - To ask questions and answer them, and to update / delete the questions / answers you have posted.
  - Search for posts based on title, owner or tags, equipped with autocomplete.

## Structure:
- API flowchart attached.
  - /
    - me: User profile (R)
    - bio: Edit user about-me (U)
    - auth
      - login
      - logout
      - register
    - auto
      - t: Tag-name autocomplete
      - u: User display-name autcomplete
      - p: Post title autocomplete
    - search
      - title: Search post by title
      - tags: Search post by single/multiple tags
    - user/{id}
      - questions: All questions posed by user
      - answers: All answers given by user 
    - qa
      - question: Pose a question
      - {id}: Gets a full qa page.
        - answer: Answer question with given id
        - update: Update post with given id
        - delete: Delete post with given id
- ORM: Support for the following
  - Prefix-based autocomplete for user display-names, post title, and tagnames : `get_all_[d|p|tag]names`.
  - Search for posts based on id, user, title or tags: `question_search_[title|owner]`, `answer_search_owner`, `post_search_many_tags`, `get_post_by_id`.
  - CRUD for posts, including ability to answer previously posed questions: `new_post`, `answer`, `update`, `delete`, `all_answers`.
  - CRUD for accounts: `make_me`, `iam`, `account_by_[id|unm]`
- Client-side:

## Software used:
- Database: PostgreSQL (https://www.postgresql.org/)
- ORM software: Diesel (https://diesel.rs) in Rust.
- Backend Web framework: Actix-web (https://actix.rs) in Rust.
- Frontend web framework: Flutter (https://flutter.dev) in Dart.

## Members:
- Kartheek Tammana (cs21btech11028): Overall testing, Frontend architecture
- Abhay Shankar K (cs21btech11001): Backend architecture, Database setup