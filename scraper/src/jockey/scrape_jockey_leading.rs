use dotenv::dotenv;
use encoding_rs::EUC_JP;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::{env, thread, time};

#[tokio::main]
async fn get_jockey_leading_html(year: u32, page: i32) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://db.netkeiba.com/?pid=jockey_leading&year={}&page={}",
        year, page
    );

    let res = reqwest::get(&url).await?;
    assert_eq!(res.status(), 200);

    dotenv().ok();
    let sleep_millisec: u64 = env::var("SLEEP")
        .expect("SLEEP must be set")
        .parse()
        .expect("SLEEP must be integer");
    thread::sleep(time::Duration::from_millis(sleep_millisec));

    let body_bytes = res.bytes().await?;

    let (html, _enc, errors) = EUC_JP.decode(&body_bytes);
    if errors {
        eprintln!("{:?}", errors);
    }
    let html = html.to_string();
    Ok(html)
}

fn html_to_jockey_ids(html: &str) -> (Vec<String>, bool) {
    let document = Document::from(html);

    let mut jockey_ids: Vec<String> = Vec::new();

    for (row, tr) in document
        .find(Class("race_table_01").descendant(Name("tr")))
        .enumerate()
    {
        if row > 1 {
            for (col, td) in tr.find(Name("td")).enumerate() {
                if col == 1 {
                    let jockey_id = td
                        .find(Name("a"))
                        .nth(0)
                        .unwrap()
                        .attr("href")
                        .unwrap()
                        .replace("/jockey/", "")
                        .replace("/", "");
                    jockey_ids.push(jockey_id);
                }
            }
        }
    }
    let mut next_flag = false;
    for a in document.find(Class("pager").descendant(Name("a"))) {
        if a.text() == "次" {
            next_flag = true;
        }
    }

    (jockey_ids, next_flag)
}

pub fn get_jockey_ids(year: u32) -> Vec<String> {
    let mut jockey_ids = Vec::new();

    let mut next_flag = true;
    let mut page = 1;

    while next_flag {
        println!("{:?}", page);
        let (ids, flag) = match get_jockey_leading_html(year, page) {
            Ok(html) => html_to_jockey_ids(&html),
            Err(e) => panic!(e),
        };
        if flag {
            page += 1;
        } else {
            next_flag = false;
        }

        jockey_ids.extend(ids.clone());
    }

    jockey_ids
}
