use mwbot::Result;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = mwbot::Bot::from_default_config().await?;
    let resp = bot
        .api()
        .get_value(&[
            ("action", "query"),
            ("pllimit", "2"),
            ("prop", "links"),
            ("titles", "Rust_(programming_language)"),
        ])
        .await?;
    for link in resp["query"]["pages"][0]["links"]
        .as_array()
        .unwrap()
        .into_iter()
    {
        println!("{}", link["title"]);
        let lead = bot
            .page(link["title"].as_str().unwrap())?
            .html()
            .await?
            .into_mutable()
            .select("section > p")
            .unwrap()
            .next()
            .unwrap()
            .text_contents();
        println!("{lead}");
    }
    Ok(())
}
