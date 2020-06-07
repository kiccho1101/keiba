use crate::race::model::Race;
use crate::utils::sleep::sleep;
use chrono::format::strftime::StrftimeItems;
use chrono::NaiveDate;
use encoding_rs::EUC_JP;
use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

/*
race_id
ex) 201905020111
- year(4): 2019
- field_id(2): 05 (東京)
- season(2): 02
- week(2): 01
- race_id(2): 11
*/

#[tokio::main]
pub async fn get_race_html(race_id: &String) -> Result<String, reqwest::Error> {
    let url = format!("https://db.netkeiba.com/race/{}/", race_id);

    let res = reqwest::get(&url).await?;
    assert_eq!(res.status(), 200);

    sleep();

    let body_bytes = res.bytes().await?;

    let (html, _enc, errors) = EUC_JP.decode(&body_bytes);
    if errors {
        eprintln!("{:?}", errors);
    }
    let html = html.into_owned();
    Ok(html)
}

pub fn race_exists(html: &str) -> bool {
    let mut exists = true;
    let document = Document::from(html);
    match document
        .find(Class("racedata").descendant(Name("h1")))
        .nth(0)
    {
        Some(_) => (),
        None => {
            exists = false;
        }
    }
    exists
}

pub fn html_to_race(race_id: &str, html: &str) -> Race {
    let document = Document::from(html);

    let name = document
        .find(Class("racedata").descendant(Name("h1")))
        .nth(0)
        .expect("Race name not found")
        .text()
        .replace(" ", "");

    let race_info_str = document
        .find(Class("racedata").descendant(Name("span")))
        .nth(0)
        .expect("Race info not found")
        .text()
        .replace("\n", "")
        .replace("/", "");

    let race_info_split = race_info_str.split("\u{a0}\u{a0}").to_owned();

    let mut race_type: Option<String> = None;
    let mut meter: Option<i32> = None;
    let mut weather: Option<String> = None;
    let mut condition: Option<String> = None;
    let mut start_time: Option<String> = None;

    for (i, race_info) in race_info_split.enumerate() {
        // race_type (芝左/芝右/ダ左/ダ右/障芝 外-内/)
        // meter
        if i == 0 {
            let meter_re: Regex = Regex::new(r"(\d{1,10})m").unwrap();
            let meter_str = String::from(
                meter_re
                    .captures(race_info)
                    .and_then(|cap| cap.get(1))
                    .expect("no meter expression found")
                    .as_str(),
            );
            race_type = Some(String::from(
                String::from(race_info)
                    .split(&meter_str)
                    .collect::<Vec<&str>>()[0],
            ));
            meter = Some(meter_str.parse().expect("meter is not integer"));
        }
        let key = String::from(String::from(race_info).split(" : ").collect::<Vec<&str>>()[0]);

        // weather
        if key == "天候" {
            weather = Some(String::from(
                String::from(race_info).split(" : ").collect::<Vec<&str>>()[1],
            ));
        }

        // condition
        if key == "芝" || key == "ダート" {
            condition = Some(String::from(
                String::from(race_info).split(" : ").collect::<Vec<&str>>()[1],
            ));
        }

        // start_time
        if key == "発走" {
            start_time = Some(String::from(
                String::from(race_info).split(" : ").collect::<Vec<&str>>()[1],
            ));
        }
    }

    let race_detail = document
        .find(Class("data_intro").descendant(Class("smalltxt")))
        .nth(0)
        .expect("no race_detail element (smalltxt) found")
        .text();

    let race_detail_main = String::from(
        race_detail
            .split("\u{a0}\u{a0}")
            .nth(0)
            .expect("no race_detail element"),
    );

    let other_detail = match race_detail.split("\u{a0}\u{a0}").nth(1) {
        Some(s) => Some(String::from(s)),
        None => None,
    };

    // race_date
    let race_date = match race_detail_main.split(" ").nth(0) {
        Some(first_str) => {
            let fmt = StrftimeItems::new("%Y-%m-%d");
            let race_date_nozfill = String::from(first_str)
                .replace("年", "-")
                .replace("月", "-")
                .replace("日", "-");
            Some(
                NaiveDate::from_ymd(
                    String::from(race_date_nozfill.split("-").nth(0).expect("year not found"))
                        .parse()
                        .expect("year is not int"),
                    String::from(
                        race_date_nozfill
                            .split("-")
                            .nth(1)
                            .expect("month not found"),
                    )
                    .parse()
                    .expect("month is not int"),
                    String::from(race_date_nozfill.split("-").nth(2).expect("day not found"))
                        .parse()
                        .expect("day is not int"),
                )
                .format_with_items(fmt.clone())
                .to_string(),
            )
        }
        None => None,
    };

    let qualifications = match race_detail_main.split(" ").nth(2) {
        Some(s) => Some(String::from(s)),
        None => None,
    };

    let race = Race {
        id: String::from(race_id),
        name: name,
        race_date: race_date,
        start_time: start_time,
        race_type: race_type,
        meter: meter,
        weather: weather,
        condition: condition,
        qualifications: qualifications,
        other_detail: other_detail,
    };
    race
}
