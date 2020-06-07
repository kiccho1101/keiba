use crate::utils::sleep::sleep;
use encoding_rs::EUC_JP;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[tokio::main]
async fn get_race_calendar_html(year: i32, month: i32) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://race.netkeiba.com/top/calendar.html?year={}&month={}",
        year, month
    );

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

fn html_to_race_list_url(html: &str) -> Vec<String> {
    let mut race_list_urls: Vec<String> = Vec::new();

    let document = Document::from(html);
    for a in document.find(Class("RaceCellBox").descendant(Name("a"))) {
        match a.attr("href") {
            Some(href) => race_list_urls.push(format!(
                "https://race.netkeiba.com/{}",
                String::from(href).replace("../", "")
            )),
            None => (),
        };
    }

    race_list_urls
}

pub fn get_race_list_urls(year: i32, month: i32) -> Vec<String> {
    let _html = match get_race_calendar_html(year, month) {
        Ok(html) => html,
        Err(e) => panic!(e),
    };
    let urls = html_to_race_list_url(&_html);
    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_race_list_from_calendar() {
        let year = 2020;
        let month = 1;
        let _html = match get_race_calendar_html(year, month) {
            Ok(html) => html,
            Err(e) => panic!(e),
        };
        let urls = html_to_race_list_url(&_html);
        println!("{:?}", urls);
        assert_eq!(
            urls,
            vec![
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200105".to_string(),
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200106".to_string(),
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200111".to_string(),
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200112".to_string(),
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200113".to_string(),
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200118".to_string(),
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200119".to_string(),
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200125".to_string(),
                "https://race.netkeiba.com/top/race_list.html?kaisai_date=20200126".to_string()
            ]
        );
    }
}
