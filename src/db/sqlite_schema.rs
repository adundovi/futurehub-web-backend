table! {
    events (id) {
        id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
        place -> Nullable<Text>,
        audience -> Nullable<Text>,
        datetime -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        slug -> Text,
        body -> Text,
        published -> Bool,
        datetime -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    posts,
);
