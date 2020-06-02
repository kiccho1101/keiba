pub async fn get_jockey_html(jockey_id: &String) -> Result<String, reqwest::Error> {
    let url = format!("https://db.netkeiba.com/jockey/{}", jockey_id);
    println!("{:?}", url);
    let res = reqwest::get(&url).await?;
    println!("Status {}", res.status());
    assert_eq!(res.status(), 200);
    let body = res.text().await?;
    Ok(body)
}
