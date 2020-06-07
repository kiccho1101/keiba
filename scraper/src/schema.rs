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

table! {
    races (id) {
        inserted_at -> Nullable<Timestamp>,
        id -> Text,
        name -> Nullable<Text>,
        race_date -> Nullable<Text>,
        start_time -> Nullable<Text>,
        race_type -> Nullable<Text>,
        meter -> Nullable<Int4>,
        weather -> Nullable<Text>,
        condition -> Nullable<Text>,
        qualifications -> Nullable<Text>,
        other_detail -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    jockeys,
    races,
);
