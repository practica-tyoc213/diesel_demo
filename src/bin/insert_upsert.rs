use diesel::pg::Pg;
use diesel::{debug_query, insert_into, RunQueryDsl};
use diesel_demo::establish_connection;
use diesel_demo::models::{NewPost, Post};
use diesel_demo::schema::posts::dsl::{id, posts};
use std::time::SystemTime;

fn main() {
    let conn = establish_connection();
    let now = SystemTime::now();
    let post = Post {
        id: 1,
        title: "title 1".into(),
        body: "body 1".into(),
        published: true,
        publish_at: Some(now),
        visit_count: Some(0),
        updated_at: Some(now),
    };
    let upsert = insert_into(posts)
        .values(&post)
        .on_conflict(id)
        .do_update()
        .set(&post);
    println!("{}", debug_query::<Pg, _>(&upsert));
    let r = upsert.execute(&conn);
    println!("{:?}", r);
}
