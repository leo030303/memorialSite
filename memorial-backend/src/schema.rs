// @generated automatically by Diesel CLI.

diesel::table! {
    anecdotes (anecdote_id) {
        anecdote_id -> Int4,
        #[max_length = 255]
        author -> Varchar,
        #[max_length = 10485760]
        content -> Varchar,
        published -> Bool,
    }
}

diesel::table! {
    condolences (condolence_id) {
        condolence_id -> Int4,
        #[max_length = 255]
        author -> Varchar,
        #[max_length = 10485760]
        content -> Varchar,
    }
}

diesel::table! {
    pictures (picture_id) {
        picture_id -> Int4,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 255]
        caption -> Varchar,
    }
}

diesel::table! {
    videos (video_id) {
        video_id -> Int4,
        #[max_length = 255]
        url -> Varchar,
        #[max_length = 255]
        title -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    anecdotes,
    condolences,
    pictures,
    videos,
);
