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
