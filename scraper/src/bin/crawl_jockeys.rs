use scraper::jockey::model::{upsert_jockeys, Jockey};
use scraper::jockey::scrape_jockey_leading;
fn main() {
    for year in (1956..1980).rev() {
        println!("Getting jockey ids from ranking of year {:?}...", year);
        let jockey_ids = scrape_jockey_leading::get_jockey_ids(year);
        println!("There are {} jockey ids found.", jockey_ids.len());
        println!("Getting data of each jockey");
        let jockey_data: Vec<Jockey> = jockey_ids
            .into_iter()
            .map(|jockey_id| Jockey::from_jockey_id(&jockey_id))
            .collect();
        println!("Upsert data into DB");
        upsert_jockeys(&jockey_data);
    }
}
