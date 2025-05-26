use std::io::{Write, stdout};

use curl::easy::Easy;

fn main() {
    let mut easy = Easy::new();
    easy.url("https://en.wikipedia.org/w/api.php?action=query&pllimit=500&titles=Rust_(programming_language)&prop=links&format=json").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
