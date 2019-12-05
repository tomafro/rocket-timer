table! {
    stopwatches (id) {
        id -> Int4,
        identifier -> Varchar,
        name -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}
