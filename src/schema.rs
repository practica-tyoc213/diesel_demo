table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        publish_at -> Nullable<Timestamp>,
        visit_count -> Nullable<Int4>,
    }
}
