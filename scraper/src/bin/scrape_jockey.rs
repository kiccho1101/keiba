use scraper::jockey::*;
fn main() {
    let jockey_id = String::from("05339");
    let body = match request::get_jockey_html(&jockey_id) {
        Ok(b) => b,
        Err(e) => panic!(e),
    };
    println!("{}", body);
}
