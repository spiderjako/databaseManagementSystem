table! {
    appointments (id) {
        id -> Int4,
        username -> Varchar,
        doctor -> Varchar,
        time_of_app -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        user_type -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    appointments,
    users,
);
