use super::schema::posts;
use chrono::NaiveDateTime;
use diesel::expression::AsExpression;
use diesel::pg::data_types::PgTimestamp;
use diesel::query_builder::{BuildQueryResult, QueryBuilder};
use diesel::sql_types::{Nullable, Timestamp, Timestamptz};
use diesel::Expression;
use std::time::SystemTime;

// if your struct has both
// #[derive(AsChangeset)] and #[derive(Identifiable)],
// you will be able to use the save_changes method
#[derive(Queryable, AsChangeset, Identifiable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_at: Option<SystemTime>,
    pub visit_count: Option<i32>,
    pub updated_at: Option<SystemTime>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub publish_at: Option<SystemTime>,
    pub visit_count: Option<i32>,
    pub updated_at: Option<SystemTime>,
}
