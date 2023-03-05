# DBMS-2 (CS3550) : Assignment 2

## Installation and execution instructions
- 
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
      - {id}
        - answer: Answer question with given id
        - update: Update post with given id
        - delete: Delete post with given id
- ORM:
- Client-side:

## Software used:
- ORM software: Diesel (https://diesel.rs) in Rust.
- Backend Web framework: Actix-web (https://actix.rs) in Rust.
- Frontend web framework: Flutter (https://flutter.dev) in Dart.

## Members:
- Kartheek Tammana (cs21btech11028): Overall testing, Frontend architecture
- Abhay Shankar K (cs21btech11001): Backend architecture