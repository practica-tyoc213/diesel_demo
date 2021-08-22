use diesel::{insert_into, select, ExpressionMethods, RunQueryDsl};
use diesel_demo::establish_connection;
use diesel_demo::models::Post;
use diesel_demo::schema::posts::columns::{body, id, title};
use diesel_demo::schema::posts::dsl::posts;
use std::time::SystemTime;

fn main() {
    let conn = establish_connection();
    let now = select(diesel::dsl::now)
        .get_result::<SystemTime>(&conn)
        .expect("doesn't work");
    println!("{:?}", now);
    let vals = vec![
        (title.eq("1"), body.eq("Sean")),
        (title.eq("2"), body.eq("Tess")),
    ];
    let inserted_posts: Vec<Post> = insert_into(posts)
        .values(vals)
        .get_results(&conn)
        .expect("An error ocurred");

    let expected_posts = vec![
        Post {
            id: 19191,
            title: "1".into(),
            body: "Sean".into(),
            published: false,
            publish_at: Some(now),
            visit_count: Some(0i32),
            updated_at: Some(now),
        },
        Post {
            id: 19192,
            title: "2".into(),
            body: "Tess".into(),
            published: false,
            publish_at: Some(now),
            visit_count: Some(0i32),
            updated_at: Some(now),
        },
    ];
    println!("{:?}", inserted_posts);

    let r = insert_into(posts)
        .values((title.eq("Ruby"), body.eq("body some")))
        .returning(id)
        .get_result::<i32>(&conn)
        .expect("not right");
    println!("------------->>>>>>>");
    println!("el ID retornado es {:?}", r);
}
