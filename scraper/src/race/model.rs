use crate::postgres::pg_connection;
use crate::race::scrape_race;
#[cfg(test)]
use crate::race::scrape_race::get_race_html;
use crate::race::scrape_race_result;
use crate::schema::{race_results, races};
use diesel::RunQueryDsl;

#[derive(Insertable, Queryable, PartialEq, Debug)]
#[table_name = "races"]
pub struct Race {
    pub id: String,
    pub name: String,
    pub race_date: Option<String>,
    pub start_time: Option<String>,
    pub race_type: Option<String>,
    pub meter: Option<i32>,
    pub weather: Option<String>,
    pub condition: Option<String>,
    pub qualifications: Option<String>,
    pub other_detail: Option<String>,
}

#[derive(Insertable, Queryable, PartialEq, Debug)]
#[table_name = "race_results"]
pub struct RaceResult {
    pub id: String,
    pub rank: i32,
    pub frame: Option<i32>,
    pub number: Option<i32>,
    pub horse_id: Option<i32>,
    pub horse_name: Option<String>,
    pub horse_age: Option<i32>,
    pub handicap: Option<f64>,
    pub jockey_id: Option<String>,
    pub jockey_name: Option<String>,
    pub time: Option<String>,
    pub rank_diff: Option<String>,
    pub time_index: Option<String>,
    pub pass_rank: Option<String>,
    pub three_furlongs: Option<f64>,
    pub ozz: Option<f64>,
    pub popularity: Option<i32>,
    pub horse_weight: Option<String>,
    pub training_time: Option<String>,
    pub house_comment: Option<String>,
    pub other: Option<String>,
    pub trainer_id: Option<String>,
    pub trainer_name: Option<String>,
    pub owner_id: Option<String>,
    pub owner_name: Option<String>,
    pub prize: Option<f64>,
}

impl Race {
    pub fn from_race_id(id: &String) -> Self {
        let html = match scrape_race::get_race_html(id) {
            Ok(html) => html,
            Err(e) => panic!(e),
        };
        scrape_race::html_to_race(id, &html)
    }

    pub fn from_html(id: &str, html: &str) -> Self {
        scrape_race::html_to_race(id, html)
    }
}

impl RaceResult {
    pub fn new(race_id: &str, rank: i32) -> Self {
        Self {
            id: String::from(race_id),
            rank: rank,
            frame: None,
            number: None,
            horse_id: None,
            horse_name: None,
            horse_age: None,
            handicap: None,
            jockey_id: None,
            jockey_name: None,
            time: None,
            rank_diff: None,
            time_index: None,
            pass_rank: None,
            three_furlongs: None,
            ozz: None,
            popularity: None,
            horse_weight: None,
            training_time: None,
            house_comment: None,
            other: None,
            trainer_id: None,
            trainer_name: None,
            owner_id: None,
            owner_name: None,
            prize: None,
        }
    }

    pub fn from_html(race_id: &str, html: &str) -> Vec<Self> {
        scrape_race_result::html_to_race_result(race_id, html)
    }
}

pub fn upsert_races(race_data: &Vec<Race>) {
    let conn = pg_connection();
    let insert_count = diesel::insert_into(races::table)
        .values(race_data)
        .on_conflict(races::id)
        .do_nothing()
        .execute(&conn);
    match insert_count {
        Ok(count) => println!("races: {} rows are inserted", count),
        Err(e) => panic!(e),
    }
}

pub fn upsert_race_results(race_result_data: &Vec<RaceResult>) {
    let conn = pg_connection();
    let insert_count = diesel::insert_into(race_results::table)
        .values(race_result_data)
        .on_conflict_do_nothing()
        .execute(&conn);
    match insert_count {
        Ok(count) => println!("race_results: {} rows are inserted", count),
        Err(e) => panic!(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_from_id1() {
        let _race_id = String::from("201905050510");
        let race = Race::from_race_id(&_race_id);
        assert_eq!(race.id, String::from("201905050510"));
        assert_eq!(race.name, String::from("ユートピアステークス"));
        assert_eq!(race.race_date, Some(String::from("2019-11-16")));
        assert_eq!(race.start_time, Some(String::from("14:55")));
        assert_eq!(race.race_type, Some(String::from("芝左")));
        assert_eq!(race.meter, Some(1600));
        assert_eq!(race.weather, Some(String::from("晴")));
        assert_eq!(race.condition, Some(String::from("良")));
        assert_eq!(race.qualifications, Some(String::from("3歳以上1600万下")));
        assert_eq!(race.other_detail, Some(String::from("(混) 牝(特指)(定量)")));
    }

    #[test]
    fn test_get_race_result() {
        let _race_id = String::from("201905050510");
        let _html = match get_race_html(&_race_id) {
            Ok(html) => html,
            Err(e) => panic!(e),
        };
        let race_results = RaceResult::from_html(&_race_id, &_html);
        assert_eq!(
            race_results[0].horse_name,
            Some(String::from("メイショウグロッケ"))
        );
    }
}
