// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (task_id) {
        task_id -> Int4,
        task -> Varchar,
        complete -> Bool,
    }
}
