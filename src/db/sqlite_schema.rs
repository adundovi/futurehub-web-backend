table! {
    categories (id) {
        id -> Integer,
        title -> Text,
        slug -> Text,
        icon -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

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
        category_id -> Integer,
        filetype -> Nullable<Text>,
        published -> Bool,
        datetime -> Timestamp,
    }
}

joinable!(repo_items -> categories (category_id));

allow_tables_to_appear_in_same_query!(
    categories,
    events,
    posts,
    repo_items,
);
