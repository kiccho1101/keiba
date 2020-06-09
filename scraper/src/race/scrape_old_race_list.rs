use crate::utils::sleep::sleep;
use encoding_rs::EUC_JP;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[tokio::main]
async fn get_old_race_list_html(url: &str) -> Result<String, reqwest::Error> {
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
    for (i, tr) in document
        .find(Class("race_table_01").descendant(Name("tr")))
        .enumerate()
    {
        if i > 0 {
            match tr.find(Name("td")).nth(1) {
                Some(td) => match td.find(Name("a")).nth(0) {
                    Some(a) => {
                        race_ids.push(
                            a.attr("href")
                                .unwrap()
                                .replace("/race/", "")
                                .replace("/", ""),
                        );
                    }
                    None => (),
                },
                None => (),
            }
        }
    }
    race_ids
}

pub fn get_race_ids(url: &str) -> Vec<String> {
    let html = match get_old_race_list_html(url) {
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
    fn test_get_race_ids_from_old_list() {
        let url = "https://db.netkeiba.com/race/sum/05/20070610";
        let html = match get_old_race_list_html(url) {
            Ok(html) => html,
            Err(e) => panic!(e),
        };
        let race_ids = html_to_race_ids(&html);
        assert_eq!(
            race_ids,
            vec![
                "200705030801".to_string(),
                "200705030802".to_string(),
                "200705030803".to_string(),
                "200705030804".to_string(),
                "200705030805".to_string(),
                "200705030806".to_string(),
                "200705030807".to_string(),
                "200705030808".to_string(),
                "200705030809".to_string(),
                "200705030810".to_string(),
                "200705030811".to_string(),
                "200705030812".to_string(),
            ]
        );
        println!("{:?}", race_ids);
    }
}
