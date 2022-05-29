table! {
    users (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
    }
}
