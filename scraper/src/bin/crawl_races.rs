use scraper::race::model::{upsert_race_results, upsert_races, Race, RaceResult};
use scraper::race::scrape_race::{get_race_html, race_exists};
use scraper::race::{scrape_old_race_list, scrape_race_calendar, scrape_race_list};
fn main() {
    for year in (2000..2008).rev() {
        for month in (1..13).rev() {
            println!("Getting race_ids of {}, {}...", year, month);
            let urls = scrape_race_calendar::get_race_list_urls(year, month);

            for url in urls {
                println!("url {:?} start", url);
                let race_ids = match year >= 2008 {
                    true => scrape_race_list::get_race_ids(&url),
                    false => scrape_old_race_list::get_race_ids(&url),
                };

                let mut race_results: Vec<RaceResult> = Vec::new();
                let mut races: Vec<Race> = Vec::new();
                for race_id in race_ids {
                    println!("{:?}", race_id);
                    let html = match get_race_html(&race_id) {
                        Ok(html) => html,
                        Err(e) => panic!(e),
                    };
                    match race_exists(&html) {
                        true => {
                            races.push(Race::from_html(&race_id, &html));
                            race_results.extend(RaceResult::from_html(&race_id, &html));
                        }
                        false => (),
                    }
                }
                println!("url {:?} end", url);

                upsert_races(&races);
                upsert_race_results(&race_results);
            }
        }
    }
}
