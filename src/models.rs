#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub passwd: String,
}

use super::schema::posts;
use super::schema::users;

#[insertable_into(posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[insertable_into(users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub passwd: &'a str,
}
