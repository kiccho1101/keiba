use crate::jockey::scrape_jockey;
use crate::postgres::pg_connection;
use crate::schema::jockeys;
use diesel::RunQueryDsl;

#[derive(Insertable, Queryable, PartialEq, Debug)]
#[table_name = "jockeys"]
pub struct Jockey {
    pub id: String,
    pub name: String,
    pub born: Option<String>,
    pub birthday: Option<String>,
    pub blood_type: Option<String>,
    pub height: Option<i32>,
    pub weight: Option<i32>,
}

impl Jockey {
    pub fn from_html(html: &str) -> Self {
        scrape_jockey::html_to_jockey(html)
    }

    pub fn from_jockey_id(jockey_id: &String) -> Self {
        let html = match scrape_jockey::get_jockey_html(jockey_id) {
            Ok(html) => html,
            Err(e) => panic!(e),
        };
        Self::from_html(&html)
    }
}

pub fn upsert(jockey_data: &Vec<Jockey>) {
    let conn = pg_connection();
    let insert_count = diesel::insert_into(jockeys::table)
        .values(jockey_data)
        .on_conflict(jockeys::id)
        .do_nothing()
        .execute(&conn);
    match insert_count {
        Ok(count) => println!("{} rows are inserted", count),
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_jockey_id_lemaire() {
        let _jockey_id = String::from("05339");
        let jockey = Jockey::from_jockey_id(&_jockey_id);
        assert_eq!(jockey.id, "05339");
        assert_eq!(jockey.born, Some("フランス".to_string()));
        assert_eq!(jockey.blood_type, Some("B型".to_string()));
        assert_eq!(jockey.height, Some(163));
        assert_eq!(jockey.weight, Some(53));
    }

    #[test]
    fn test_from_jockey_id_yutaka() {
        let _jockey_id = String::from("00666");
        let jockey = Jockey::from_jockey_id(&_jockey_id);
        assert_eq!(jockey.id, "00666");
        assert_eq!(jockey.born, Some("京都府".to_string()));
        assert_eq!(jockey.blood_type, Some("O型".to_string()));
        assert_eq!(jockey.height, Some(170));
        assert_eq!(jockey.weight, Some(51));
    }

    #[test]
    fn test_from_jockey_id_murphy() {
        let _jockey_id = String::from("05575");
        let jockey = Jockey::from_jockey_id(&_jockey_id);
        assert_eq!(jockey.id, "05575");
        assert_eq!(jockey.born, Some("アイルランド".to_string()));
        assert_eq!(jockey.blood_type, None);
        assert_eq!(jockey.height, Some(165));
        assert_eq!(jockey.weight, Some(53));
    }

    #[test]
    fn test_from_html_file() {
        let _html = std::fs::read_to_string("./src/jockey/jockey.html")
            .expect("Unable to read jockey.html");
        let jockey = Jockey::from_html(&_html);
        assert_eq!(jockey.id, "05339");
        assert_eq!(jockey.born, Some("フランス".to_string()));
        assert_eq!(jockey.blood_type, Some("B型".to_string()));
        assert_eq!(jockey.height, Some(163));
        assert_eq!(jockey.weight, Some(53));
    }
}
