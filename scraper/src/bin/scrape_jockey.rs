use futures::executor::block_on;
use scraper::jockey::*;
fn main() {
    let jockey_id = String::from("05339");
    let body = request::get_jockey_html(&jockey_id);
    block_on(body);
}
