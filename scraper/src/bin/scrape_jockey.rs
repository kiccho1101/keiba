use diesel::insert_into;
use diesel::prelude::*;
use scraper::jockey::models::Jockey;
use scraper::postgres::pg_connection;
use scraper::schema::jockeys;

fn main() {
    let conn = pg_connection();

    let js: Vec<Jockey> = vec!["05339", "00666"]
        .into_iter()
        .map(|jockey_id| Jockey::from_jockey_id(&jockey_id.to_string()))
        .collect();

    insert_into(jockeys::table).values(&js).execute(&conn);
}
