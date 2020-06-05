use encoding_rs::EUC_JP;
use select::document::Document;
use select::predicate::{Class, Name};
use std::fs::File;
use std::io::Write;

pub struct Jockey {
    pub id: String,
    pub born: String,
    pub blood_type: String,
    pub height: i16,
    pub weight: i16,
}

fn get_td(table: select::node::Node, th: &str) -> String {
    table
        .find(Name("tr"))
        .find(|n| n.find(Name("th")).nth(0).unwrap().text() == th)
        .unwrap()
        .find(Name("td"))
        .nth(0)
        .unwrap()
        .text()
}

impl Jockey {
    #[tokio::main]
    pub async fn get_jockey_html(jockey_id: &String) -> Result<String, reqwest::Error> {
        let url = format!("https://db.netkeiba.com/jockey/profile/{}/", jockey_id);

        let res = reqwest::get(&url).await?;
        assert_eq!(res.status(), 200);

        let body_bytes = res.bytes().await?;

        let (html, _enc, errors) = EUC_JP.decode(&body_bytes);
        if errors {
            eprintln!("{:?}", errors);
        }
        let html = html.into_owned();

        let mut file = File::create("./src/jockey/jockey_scraped.html")
            .expect("could not create jockey_scraped.html");
        file.write_all(html.as_bytes())
            .expect("could not write to jockey_scraped.html");
        Ok(html)
    }

    pub fn from_html(html: &str) -> Jockey {
        let document = Document::from(html);

        let id = document
            .find(Name("a"))
            .find(|n| n.text() == "近走成績")
            .unwrap()
            .attr("href")
            .unwrap()
            .replace("/jockey/", "")
            .replace("/", "");

        let table = document.find(Class("race_table_01")).nth(0).unwrap();

        let born = get_td(table, "出身地");
        let blood_type = get_td(table, "血液型");
        let height: i16 = get_td(table, "身長").replace("cm", "").parse().unwrap();
        let weight: i16 = get_td(table, "体重").replace("kg", "").parse().unwrap();

        Jockey {
            id: id,
            born: born,
            blood_type: blood_type,
            height: height,
            weight: weight,
        }
    }

    pub fn from_jockey_id(jockey_id: &String) -> Jockey {
        let html = match Jockey::get_jockey_html(jockey_id) {
            Ok(html) => html,
            Err(e) => panic!(e),
        };
        Jockey::from_html(&html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_jockey_id_lemaire() {
        let _jockey_id = String::from("05339");
        let jockey = Jockey::from_jockey_id(&_jockey_id);
        assert_eq!(jockey.id, "05339");
        assert_eq!(jockey.born, "フランス");
        assert_eq!(jockey.blood_type, "B型");
        assert_eq!(jockey.height, 163);
        assert_eq!(jockey.weight, 53);
    }

    #[test]
    fn from_jockey_id_yutaka() {
        let _jockey_id = String::from("00666");
        let jockey = Jockey::from_jockey_id(&_jockey_id);
        assert_eq!(jockey.id, "00666");
        assert_eq!(jockey.born, "京都府");
        assert_eq!(jockey.blood_type, "O型");
        assert_eq!(jockey.height, 170);
        assert_eq!(jockey.weight, 51);
    }

    #[test]
    fn from_html_file() {
        let _html = std::fs::read_to_string("./src/jockey/jockey.html")
            .expect("Unable to read jockey.html");
        let jockey = Jockey::from_html(&_html);
        assert_eq!(jockey.id, "05339");
        assert_eq!(jockey.born, "フランス");
        assert_eq!(jockey.blood_type, "B型");
        assert_eq!(jockey.height, 163);
        assert_eq!(jockey.weight, 53);
    }
}
