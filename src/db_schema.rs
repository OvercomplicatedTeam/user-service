table! {
    parkings (parking_id) {
        parking_id -> Int4,
        name -> Text,
        password -> Text,
        admin_id -> Int4,
    }
}

table! {
    parkings_consumers (parking_id, consumer_id) {
        parking_id -> Int4,
        consumer_id -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        login -> Nullable<Text>,
        password -> Nullable<Text>,
    }
}

joinable!(parkings -> users (admin_id));
joinable!(parkings_consumers -> parkings (parking_id));
joinable!(parkings_consumers -> users (consumer_id));

allow_tables_to_appear_in_same_query!(
    parkings,
    parkings_consumers,
    users,
);
