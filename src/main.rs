use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LinksResponse {
    page_url: String
}

#[tokio::main]
async fn main() {
    use std::io::{stdin,stdout,Write};
    let mut search_url = String::new();
    print!("Enter music URL:\n➡️ ");
    let _ = stdout().flush();
    stdin().read_line(&mut search_url).expect("Did not enter a correct string");

    let response = reqwest::get(build_url(&search_url))
        .await
        .unwrap();
    
    if response.status().is_success() {
        let links = response
        .json::<LinksResponse>()
        .await
        .unwrap();

        println!();
        println!("Success ✅");
        println!("{}", links.page_url);
        println!("Song.link URL copied to your clipboard");
    } else {
        println!("❌ Error, check your search URL and try again")
    }
}

fn build_url(search_url: &str) -> String {
    let client = Client::new();
    let request = client
        .get("https://api.song.link/v1-alpha.1/links")
        .query(&[("url", search_url)])
        .build();

    match request {
        Ok(result) => return result.url().to_string(),
        Err(_) => panic!("Problem encoding URL"),
    };
}