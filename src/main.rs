use mwapi::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // let client = Client::builder("https://en.wikipedia.org/w/api.php")
    //     .set_user_agent("mwapi demo")
    //     .build()
    //     .await?;
    let bot = mwbot::Bot::from_default_config().await.unwrap();

    let resp = bot
        .api()
        .get_value(&[
            ("action", "query"),
            ("pllimit", "2"),
            ("prop", "links"),
            ("titles", "Rust_(programming_language)"),
        ])
        .await?;
    // println!("{resp}");
    for link in resp["query"]["pages"][0]["links"]
        .as_array()
        .unwrap()
        .into_iter()
    {
        println!("{}", link["title"]);
    }
    Ok(())
}
