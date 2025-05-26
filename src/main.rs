use mwbot::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let bot = mwbot::Bot::from_default_config().await.unwrap();
    let resp = bot
        .api()
        .get_value(&[
            ("action", "query"),
            ("pllimit", "2"),
            ("prop", "links"),
            ("titles", "Rust_(programming_language)"),
        ])
        .await
        .unwrap();
    for link in resp["query"]["pages"][0]["links"]
        .as_array()
        .unwrap()
        .into_iter()
    {
        println!("{}", link["title"]);
    }
    Ok(())
}
