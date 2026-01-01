// @generated automatically by Diesel CLI.

diesel::table! {
    definitions (id) {
        id -> Int4,
        term -> Varchar,
        #[max_length = 1000]
        formal_def -> Varchar,
        #[max_length = 1000]
        useful_def -> Varchar,
        #[max_length = 1000]
        simple_def -> Varchar,
        date_created -> Timestamp,
        date_updated -> Timestamp,
    }
}
