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
        body -> Nullable<Text>,
        published -> Bool,
        datetime -> Timestamp,
    }
}

table! {
    repo_items (id) {
        id -> Integer,
        title -> Text,
        slug -> Text,
        filepath -> Text,
        description -> Nullable<Text>,
        category -> Nullable<Text>,
        filetype -> Nullable<Text>,
        published -> Bool,
        datetime -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    posts,
    repo_items,
);
