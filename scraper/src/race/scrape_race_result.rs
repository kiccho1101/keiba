use crate::race::model::RaceResult;
use select::document::Document;
use select::predicate::{Class, Name};

pub fn html_to_race_result(race_id: &str, html: &str) -> Vec<RaceResult> {
    let document = Document::from(html);

    let mut race_results: Vec<RaceResult> = Vec::new();

    match document.find(Class("race_table_01")).nth(0) {
        Some(table) => {
            let theads = table.find(Name("tr")).nth(0).expect("no theads found");
            for (i, row) in table.find(Name("tr")).enumerate() {
                if i > 0 {
                    match row.find(Name("td")).nth(0) {
                        Some(td) => {
                            match td.text().parse::<i32>() {
                                Ok(rank) => {
                                    let mut race_result = RaceResult::new(race_id, rank);
                                    for (j, td) in row.find(Name("td")).enumerate() {
                                        let th = theads
                                            .find(Name("th"))
                                            .nth(j)
                                            .expect("no th found")
                                            .text()
                                            .replace(" ", "")
                                            .replace("\n", "")
                                            .replace("\t", "");

                                        let td_str = td
                                            .text()
                                            .replace(" ", "")
                                            .replace("\t", "")
                                            .replace("\n", "");
                                        if th == "枠番" {
                                            race_result.frame = Some(td_str.parse().expect(
                                                &format!("failed to parse frame: {}", td_str),
                                            ));
                                        } else if th == "馬番" {
                                            race_result.number = Some(td_str.parse().expect(
                                                &format!("failed to parse number: {}", td_str),
                                            ));
                                        } else if th == "馬名" {
                                            race_result.horse_name = Some(td_str.replace(" ", ""));
                                            match td.find(Name("a")).nth(0) {
                                                Some(a) => {
                                                    race_result.horse_id = Some(
                                                        a.attr("href")
                                                            .expect("horse_id not found")
                                                            .replace("/horse/", "")
                                                            .replace("/", "")
                                                            .parse()
                                                            .expect("horse_id is not int"),
                                                    );
                                                }
                                                None => (),
                                            }
                                        } else if th == "性齢" {
                                            let age_re = regex::Regex::new(r"(\d{1,2})").unwrap();
                                            race_result.horse_age = Some(
                                                String::from(
                                                    age_re
                                                        .captures(&td_str)
                                                        .and_then(|cap| cap.get(1))
                                                        .expect("no age found")
                                                        .as_str(),
                                                )
                                                .parse()
                                                .unwrap(),
                                            );
                                        } else if th == "斤量" {
                                            race_result.handicap = Some(td_str.parse().expect(
                                                &format!("couln't parse handicap: {}", td_str),
                                            ));
                                        } else if th == "騎手" {
                                            race_result.jockey_name = Some(td_str.replace(" ", ""));
                                            match td.find(Name("a")).nth(0) {
                                                Some(a) => {
                                                    race_result.jockey_id = Some(
                                                        a.attr("href")
                                                            .expect("jockey_id not found")
                                                            .replace("/jockey/", "")
                                                            .replace("/", ""),
                                                    );
                                                }
                                                None => (),
                                            }
                                        } else if th == "タイム" {
                                            race_result.time = Some(td_str);
                                        } else if th == "着差" {
                                            race_result.rank_diff = Some(td_str);
                                        } else if th == "通過" {
                                            race_result.pass_rank = Some(td_str);
                                        } else if th == "上り" {
                                            race_result.three_furlongs = Some(
                                                td_str
                                                    .replace(" ", "")
                                                    .parse()
                                                    .expect("failed to parse three_furlongs"),
                                            );
                                        } else if th == "単勝" {
                                            race_result.ozz = Some(
                                                td_str
                                                    .replace(" ", "")
                                                    .parse()
                                                    .expect("failed to parse ozz"),
                                            );
                                        } else if th == "人気" {
                                            race_result.popularity = Some(
                                                td_str
                                                    .replace(" ", "")
                                                    .parse()
                                                    .expect("failed to parse popularity"),
                                            );
                                        } else if th == "馬体重" {
                                            race_result.horse_weight = Some(td_str);
                                        } else if th == "調教師" {
                                            race_result.trainer_name =
                                                Some(td_str.replace(" ", ""));
                                            match td.find(Name("a")).nth(0) {
                                                Some(a) => {
                                                    race_result.trainer_id = Some(
                                                        a.attr("href")
                                                            .expect("trainer_id not found")
                                                            .replace("/trainer/", "")
                                                            .replace("/", ""),
                                                    );
                                                }
                                                None => (),
                                            }
                                        } else if th == "馬主" {
                                            race_result.owner_name = Some(td_str.replace(" ", ""));
                                            match td.find(Name("a")).nth(0) {
                                                Some(a) => {
                                                    race_result.owner_id = Some(
                                                        a.attr("href")
                                                            .expect("owner_id not found")
                                                            .replace("/owner/", "")
                                                            .replace("/", ""),
                                                    );
                                                }
                                                None => (),
                                            }
                                        } else if th == "賞金(万円)" {
                                            match td_str != "" {
                                                true => {
                                                    race_result.prize = Some(
                                                        td.text()
                                                            .replace(" ", "")
                                                            .replace(",", "")
                                                            .parse()
                                                            .expect("failed to parse prize"),
                                                    );
                                                }
                                                false => (),
                                            }
                                        }
                                    }
                                    race_results.push(race_result);
                                }
                                Err(_) => (),
                            }
                        }
                        None => (),
                    }
                }
            }
        }
        None => (),
    }

    race_results
}
