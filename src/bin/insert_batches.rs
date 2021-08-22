use diesel::pg::Pg;
use diesel::{debug_query, insert_into, ExpressionMethods, RunQueryDsl};
use diesel_demo::establish_connection;
use diesel_demo::models::NewPost;
use diesel_demo::schema::posts::columns::{body, title};
use diesel_demo::schema::posts::dsl::posts;

fn main() {
    let con = establish_connection();
    let records = &vec![
        (
            title.eq("un title batch"),
            body.eq("body something it is x"),
        ),
        (
            title.eq("this is a title or something"),
            body.eq("body xyz"),
        ),
        (title.eq("Title xyz"), body.eq("body xyz")),
    ];
    let x = insert_into(posts).values(records);
    println!("post are {}", debug_query::<Pg, _>(&x));
    x.execute(&con);

    let json = r#"[
        { "title": "Sean", "body": "Black" },
        { "title": "Tess", "body": "Brown" }
    ]"#;
    let posts_form = serde_json::from_str::<Vec<NewPost>>(json).expect("Not good");

    let r = insert_into(posts)
        .values(&posts_form)
        .execute(&con)
        .expect("not good");
    println!("{:?}", r);
}
