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
    course_events (id) {
        id -> Integer,
        course_id -> Nullable<Integer>,
        event_id -> Nullable<Integer>,
    }
}

table! {
    course_users (id) {
        id -> Integer,
        course_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        join_date -> Timestamp,
        leave_date -> Nullable<Timestamp>,
        score -> Nullable<Integer>,
        attendance -> Nullable<Integer>,
        note -> Nullable<Text>,
    }
}

table! {
    courses (id) {
        id -> Integer,
        code -> Text,
        title -> Text,
        description -> Nullable<Text>,
        creation_date -> Timestamp,
        cert_template -> Nullable<Text>,
        lecturer -> Nullable<Text>,
        organizer -> Nullable<Text>,
        lectures -> Nullable<Integer>,
        students -> Nullable<Integer>,
        max_students -> Nullable<Integer>,
        finished -> Bool,
        published -> Bool,
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
        filehash -> Nullable<Text>,
        filesize -> Nullable<BigInt>,
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

joinable!(course_events -> courses (course_id));
joinable!(course_events -> events (event_id));
joinable!(course_users -> courses (course_id));
joinable!(course_users -> users (user_id));
joinable!(login_history -> users (user_id));
joinable!(repo_items -> categories (category_id));

allow_tables_to_appear_in_same_query!(
    categories,
    course_events,
    course_users,
    courses,
    events,
    login_history,
    posts,
    repo_items,
    users,
);
