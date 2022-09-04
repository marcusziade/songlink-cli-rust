use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Serialize, Deserialize};
use reqwest::Client;

// The LinksResponse model
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SongLinks {
    page_url: String
}

// Entrypoint of the app. 
// Gets the user query, and fetches the song.link URL.
#[tokio::main]
async fn main() {
    get_links(&get_user_query()).await;
}

// Asks the user to paste and confirm a music service URL, formats, and returns the query.
fn get_user_query() -> String {
    use std::io::{stdin,stdout,Write};
    let mut query = String::new();
    print!("Enter music URL:\n➡️ ");
    let _ = stdout().flush();
    stdin().read_line(&mut query).expect("Did not enter a correct string");
    return query;
}

// Takes a music service URL as input.
// checks if the response is succesful, decodes the json,
// copies the generated song.link URL to the clipboard and prints it to the interface.
async fn get_links(query: &str) {
    let response = reqwest::get(build_url(&query))
    .await
    .unwrap();

    if response.status().is_success() {
        let links = response
        .json::<SongLinks>()
        .await
        .unwrap();

        let songlink = links.page_url;
        println!("\nSuccess ✅\n{songlink}\nSong.link URL copied to your clipboard");

        let mut context: ClipboardContext = ClipboardProvider::new().unwrap();
        context.set_contents(songlink.to_owned()).unwrap();
    } else {
        println!("❌ Error, check your search URL and try again")
    }
}

// Takes in a music service URL, builds the song.link API query, and returns it as a string.
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