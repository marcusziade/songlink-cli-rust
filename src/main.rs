use std::io::{stdin, stdout, Write};

use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// The LinksResponse model
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SongLinks {
    page_url: String,
}

// Entrypoint of the app.
// Gets the user query, and fetches the song.link URL.
#[tokio::main]
async fn main() {
    loop {
        let query = match get_user_query() {
            Ok(query) => query,
            Err(e) => {
                eprintln!("ERROR: {e}");
                break;
            }
        };

        if let Err(err) = get_links(&query).await {
            eprintln!("ERROR: {err}");
        }
    }
}

// Asks the user to paste and confirm a music service URL, formats, and returns the query.
fn get_user_query() -> Result<String> {
    print!("Enter music URL:\n➡️ ");
    stdout().flush()?;

    let mut query = String::new();
    stdin().read_line(&mut query)?;
    Ok(query)
}

// Takes a music service URL as input.
// checks if the response is succesful, decodes the json,
// copies the generated song.link URL to the clipboard and prints it to the interface.
async fn get_links(query: &str) -> Result<()> {
    let response = reqwest::get(build_url(&query)?).await?;

    if response.status().is_success() {
        let links = response.json::<SongLinks>().await?;

        let songlink = links.page_url;
        println!("Success ✅\n{songlink}\nSong.link URL copied to your clipboard");

        let mut context: ClipboardContext = ClipboardProvider::new().map_err(|e| anyhow::anyhow!("{e}"))?;
        context.set_contents(songlink).map_err(|e| anyhow::anyhow!("{e}"))?;

        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "❌ Error, check your search URL and try again"
        ))
    }
}

// Takes in a music service URL, builds the song.link API query, and returns it as a string.
fn build_url(search_url: &str) -> Result<String> {
    let client = Client::new();
    let url =client
        .get("https://api.song.link/v1-alpha.1/links")
        .query(&[("url", search_url)])
        .build()?
        .url()
        .to_string();
        
    Ok(url)
}