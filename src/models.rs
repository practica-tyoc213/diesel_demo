use super::schema::posts;
use serde::Deserialize;
use std::time::SystemTime;

// if your struct has both
// #[derive(AsChangeset)] and #[derive(Identifiable)],
// you will be able to use the save_changes method
#[derive(Insertable, Queryable, AsChangeset, Identifiable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_at: Option<SystemTime>,
    pub visit_count: Option<i32>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub publish_at: Option<SystemTime>,
    pub visit_count: Option<i32>,
    pub updated_at: Option<SystemTime>,
}
