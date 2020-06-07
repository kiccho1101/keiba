use crate::race::scrape_race;
use crate::schema::races;

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

impl Race {
    pub fn from_id(id: &String) -> Self {
        let html = match scrape_race::get_race_html(id) {
            Ok(html) => html,
            Err(e) => panic!(e),
        };
        scrape_race::html_to_race(id, &html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_from_id1() {
        let _race_id = String::from("201905050510");
        let race = Race::from_id(&_race_id);
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
}
