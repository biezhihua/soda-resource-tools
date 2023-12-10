// @generated automatically by Diesel CLI.

diesel::table! {
    table_management_rule (id) {
        id -> Integer,
        src -> Text,
        target -> Text,
        content_type -> Text,
        mode -> Text,
        period -> Text,
        status -> Text,
        monitor -> Text
    }
}

diesel::table! {
    table_user (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
        permission -> Text,
        email -> Nullable<Text>,
        avatar -> Nullable<Text>,
    }
}
