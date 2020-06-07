use crate::utils::sleep::sleep;
use encoding_rs::EUC_JP;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[tokio::main]
async fn get_race_list_html(url: &str) -> Result<String, reqwest::Error> {
    let url = String::from(url.replace("race_list", "race_list_sub"));

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

fn html_to_race_ids(html: &str) -> Vec<String> {
    let mut race_ids: Vec<String> = Vec::new();

    let document = Document::from(html);
    for a in document.find(Class("RaceList_DataItem").descendant(Name("a"))) {
        if a.attr("class").expect("no class in a") == "" {
            let race_id_re = regex::Regex::new(r"race_id=(\d{12})").unwrap();
            race_ids.push(String::from(
                race_id_re
                    .captures(a.attr("href").unwrap())
                    .and_then(|cap| cap.get(1))
                    .expect("no race_id found")
                    .as_str(),
            ));
        }
    }
    race_ids
}

pub fn get_race_ids(url: &str) -> Vec<String> {
    let html = match get_race_list_html(url) {
        Ok(html) => html,
        Err(e) => panic!(e),
    };
    let race_ids = html_to_race_ids(&html);
    race_ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_race_ids_from_list() {
        let url = "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200105";
        let html = match get_race_list_html(url) {
            Ok(html) => html,
            Err(e) => panic!(e),
        };
        let race_ids = html_to_race_ids(&html);
        assert_eq!(
            race_ids,
            vec![
                "202006010101".to_string(),
                "202006010102".to_string(),
                "202006010103".to_string(),
                "202006010104".to_string(),
                "202006010105".to_string(),
                "202006010106".to_string(),
                "202006010107".to_string(),
                "202006010108".to_string(),
                "202006010109".to_string(),
                "202006010110".to_string(),
                "202006010111".to_string(),
                "202006010112".to_string(),
                "202008010101".to_string(),
                "202008010102".to_string(),
                "202008010103".to_string(),
                "202008010104".to_string(),
                "202008010105".to_string(),
                "202008010106".to_string(),
                "202008010107".to_string(),
                "202008010108".to_string(),
                "202008010109".to_string(),
                "202008010110".to_string(),
                "202008010111".to_string(),
                "202008010112".to_string()
            ]
        );
        println!("{:?}", race_ids);
    }
}
