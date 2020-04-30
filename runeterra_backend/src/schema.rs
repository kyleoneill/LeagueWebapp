table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hashed_password -> Text,
        creation_date -> Nullable<Text>,
    }
}
