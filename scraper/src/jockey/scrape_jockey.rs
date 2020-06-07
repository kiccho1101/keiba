use crate::jockey::model::Jockey;
use crate::utils::sleep::sleep;
use encoding_rs::EUC_JP;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

#[tokio::main]
pub async fn get_jockey_html(jockey_id: &String) -> Result<String, reqwest::Error> {
    let url = format!("https://db.netkeiba.com/jockey/profile/{}/", jockey_id);

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

fn get_td(table: select::node::Node, th: &str) -> Option<String> {
    let tr = table.find(Name("tr")).find(|n| {
        n.find(Name("th"))
            .nth(0)
            .expect(&format!("couldn't find th '{}'", th))
            .text()
            == th
    });

    match tr {
        Some(tr) => Some(
            tr.find(Name("td"))
                .nth(0)
                .expect(&format!("couldn't find td '{}'", th))
                .text(),
        ),
        None => None,
    }
}

pub fn html_to_jockey(html: &str) -> Jockey {
    let document = Document::from(html);

    let id = document
        .find(Name("a"))
        .find(|n| n.text() == "近走成績")
        .expect("近走成績 is not found")
        .attr("href")
        .expect("attr 'href' is not found")
        .replace("/jockey/", "")
        .replace("/", "");

    let name = String::from(
        document
            .find(Attr("id", "db_main_box").descendant(Name("h1")))
            .nth(0)
            .expect("0th of h1 is not found")
            .text()
            .replace("\n", "")
            .split("\u{a0}")
            .nth(0)
            .expect("0th of split not found"),
    );

    let birthday_node = document
        .find(Attr("id", "db_main_box").descendant(Class("txt_01")))
        .nth(0);
    let birthday = match birthday_node {
        Some(node) => Some(String::from(
            node.text().split("\n").collect::<Vec<&str>>()[1],
        )),
        None => None,
    };

    let mut born: Option<String> = None;
    let mut blood_type: Option<String> = None;
    let mut height: Option<i32> = None;
    let mut weight: Option<i32> = None;

    let table_node = document.find(Class("race_table_01")).nth(0);
    if table_node.is_some() {
        let table = table_node.unwrap();
        born = get_td(table, "出身地");
        blood_type = get_td(table, "血液型");

        let height_node = get_td(table, "身長");
        height = match height_node {
            Some(node) => Some(
                node.replace("cm", "")
                    .parse()
                    .expect("failed to parse height"),
            ),
            None => None,
        };

        let weight_node = get_td(table, "体重");
        weight = match weight_node {
            Some(node) => Some(
                node.replace("kg", "")
                    .parse()
                    .expect("failed to parse weight"),
            ),
            None => None,
        };
    }

    let jockey = Jockey {
        id: id,
        name: name,
        born: born,
        birthday: birthday,
        blood_type: blood_type,
        height: height,
        weight: weight,
    };
    jockey
}

#[tokio::main]
pub async fn get_jockey_leading_html(year: i32, page: i32) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://db.netkeiba.com/?pid=jockey_leading&year={}&page={}",
        year, page
    );

    let res = reqwest::get(&url).await?;
    assert_eq!(res.status(), 200);

    sleep();

    let body_bytes = res.bytes().await?;

    let (html, _enc, errors) = EUC_JP.decode(&body_bytes);
    if errors {
        eprintln!("{:?}", errors);
    }
    let html = html.to_string();
    Ok(html)
}
