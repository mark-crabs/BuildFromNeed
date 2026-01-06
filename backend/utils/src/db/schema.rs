// @generated automatically by Diesel CLI.

diesel::table! {
    featured (id) {
        id -> Int8,
        expired -> Bool,
        expiring_date -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    problem (id) {
        id -> Int8,
        anonymous -> Bool,
        user_id -> Int8,
        title -> Text,
        description -> Text,
        flag -> Text,
        featured_id -> Nullable<Int8>,
        category -> Text,
        sub_category -> Nullable<Text>,
        status -> Text,
        public -> Bool,
        archive -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    problem_favourite (id) {
        id -> Int8,
        user_id -> Int8,
        problem_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    problem_like (id) {
        id -> Int8,
        option -> Text,
        problem_id -> Int8,
        user_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    problem_view (id) {
        id -> Int8,
        user_id -> Int8,
        problem_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    solution (id) {
        id -> Int8,
        content -> Text,
        user_id -> Int8,
        problem_id -> Int8,
        solution_type -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    solution_favourite (id) {
        id -> Int8,
        user_id -> Int8,
        solution_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    solution_like (id) {
        id -> Int8,
        option -> Text,
        solution_id -> Int8,
        user_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        name -> Nullable<Text>,
        give_name -> Nullable<Text>,
        email -> Nullable<Text>,
        email_verified -> Nullable<Bool>,
        picture -> Nullable<Text>,
        role -> Text,
        archive -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(problem -> featured (featured_id));
diesel::joinable!(problem_favourite -> problem (problem_id));
diesel::joinable!(problem_favourite -> users (user_id));
diesel::joinable!(problem_like -> problem (problem_id));
diesel::joinable!(problem_like -> users (user_id));
diesel::joinable!(problem_view -> problem (problem_id));
diesel::joinable!(problem_view -> users (user_id));
diesel::joinable!(solution -> problem (problem_id));
diesel::joinable!(solution -> users (user_id));
diesel::joinable!(solution_favourite -> solution (solution_id));
diesel::joinable!(solution_favourite -> users (user_id));
diesel::joinable!(solution_like -> solution (solution_id));
diesel::joinable!(solution_like -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    featured,
    problem,
    problem_favourite,
    problem_like,
    problem_view,
    solution,
    solution_favourite,
    solution_like,
    users,
);
