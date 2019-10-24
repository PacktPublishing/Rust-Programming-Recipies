table! {
    polls (id) {
        id -> Int4,
        question -> Varchar,
        options -> Varchar,
        owner -> Nullable<Int4>,
    }
}

table! {
    responses (poll_id, user_id) {
        poll_id -> Int4,
        user_id -> Int4,
        selected -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

joinable!(polls -> users (owner));
joinable!(responses -> polls (poll_id));
joinable!(responses -> users (user_id));

allow_tables_to_appear_in_same_query!(
    polls,
    responses,
    users,
);
