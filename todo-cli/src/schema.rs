// @generated automatically by Diesel CLI.

diesel::table! {
    Category (id) {
        id -> Nullable<Integer>,
        title -> Text,
        color -> Nullable<Text>,
        emoji -> Nullable<Text>,
    }
}

diesel::table! {
    Priority (id) {
        id -> Nullable<Integer>,
        title -> Text,
        color -> Text,
    }
}

diesel::table! {
    Status (id) {
        id -> Nullable<Integer>,
        title -> Text,
        color -> Text,
    }
}

diesel::table! {
    Tag (id) {
        id -> Nullable<Integer>,
        title -> Text,
        color -> Nullable<Text>,
    }
}

diesel::table! {
    Task (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Text,
        created_at -> Text,
        last_updated -> Text,
        priority -> Text,
        due_date -> Text,
        status_id -> Integer,
        category_id -> Integer,
    }
}

diesel::table! {
    TaskTag (task_id, tag_id) {
        task_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::joinable!(Task -> Category (category_id));
diesel::joinable!(Task -> Status (status_id));
diesel::joinable!(TaskTag -> Tag (tag_id));
diesel::joinable!(TaskTag -> Task (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    Category,
    Priority,
    Status,
    Tag,
    Task,
    TaskTag,
);
