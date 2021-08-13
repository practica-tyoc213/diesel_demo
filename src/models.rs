#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_at: Option<SystemTime>,
    pub visit_count: Option<i64>,
}
use super::schema::posts;
use diesel::dsl::Nullable;
use std::time::SystemTime;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub publish_at: Option<SystemTime>,
    pub visit_count: Option<i64>,
}
