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
    race_results (id, rank) {
        id -> Text,
        rank -> Int4,
        frame -> Nullable<Int4>,
        number -> Nullable<Int4>,
        horse_id -> Nullable<Int4>,
        horse_name -> Nullable<Text>,
        horse_age -> Nullable<Int4>,
        handicap -> Nullable<Float8>,
        jockey_id -> Nullable<Text>,
        jockey_name -> Nullable<Text>,
        time -> Nullable<Text>,
        rank_diff -> Nullable<Text>,
        time_index -> Nullable<Text>,
        pass_rank -> Nullable<Text>,
        three_furlongs -> Nullable<Float8>,
        ozz -> Nullable<Float8>,
        popularity -> Nullable<Int4>,
        horse_weight -> Nullable<Text>,
        training_time -> Nullable<Text>,
        house_comment -> Nullable<Text>,
        other -> Nullable<Text>,
        trainer_id -> Nullable<Text>,
        trainer_name -> Nullable<Text>,
        owner_id -> Nullable<Text>,
        owner_name -> Nullable<Text>,
        prize -> Nullable<Float8>,
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

allow_tables_to_appear_in_same_query!(jockeys, race_results, races,);
