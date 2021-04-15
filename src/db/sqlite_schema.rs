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
    login_history (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        login_timestamp -> Timestamp,
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

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Nullable<Text>,
        login_session -> Nullable<Text>,
        oib -> Nullable<Text>,
        name -> Nullable<Text>,
        surname -> Nullable<Text>,
        address -> Nullable<Text>,
        phone -> Nullable<Text>,
        gender -> Nullable<Text>,
        birthday -> Nullable<Timestamp>,
        creation_date -> Timestamp,
    }
}

joinable!(login_history -> users (user_id));
joinable!(repo_items -> categories (category_id));

allow_tables_to_appear_in_same_query!(
    categories,
    events,
    login_history,
    posts,
    repo_items,
    users,
);
