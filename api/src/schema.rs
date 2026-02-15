// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Text,
        completed -> Bool,
    }
}
