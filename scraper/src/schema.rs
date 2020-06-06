table! {
    jockeys (id) {
        id -> Text,
        name -> Nullable<Text>,
        born -> Nullable<Text>,
        birthday -> Nullable<Text>,
        blood_type -> Nullable<Text>,
        height -> Nullable<Int4>,
        weight -> Nullable<Int4>,
        inserted_at -> Nullable<Timestamp>,
    }
}
