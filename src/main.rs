

const BUNGIE_ROOT: &str = "https://www.bungie.net/Platform";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    debug("https://httpbin.org/ip").await?;

    let key: BungieKey = grab_api_key()?;
    // println!("using key: {}", key.0);

    let mut headers: HeaderMap = HeaderMap::new();
    let mut key_secret: header::HeaderValue = header::HeaderValue::from_str(&key.0).expect("failed to put the api key");
    key_secret.set_sensitive(true);
    headers.append("X-API-Key", key_secret);

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    dbg_bungie(&client, "/Destiny2/Manifest/").await?;

    // dgb_player_grab(&client, "/Destiny2/SearchDestinyPlayerByBungieName/All/").await?;

    Ok(())
}

async fn dbg_bungie(client: &Client, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = client.get(BUNGIE_ROOT.to_owned() + path)
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", manifest);

    Ok(())
}

async fn dgb_player_grab(client: &Client, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // look for how to set body
    let manifest = client.get(BUNGIE_ROOT.to_owned() + path)
    .send()
    .await?
    .text()
    .await?;

    println!("{:?}", manifest);

    Ok(())
}

async fn debug(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(path)
        .await?
        .text()
        .await?;
    println!("{resp}");
    Ok(())
}

use std::fs;

use reqwest::{ClientBuilder, header::{HeaderMap, self}, Client};

struct BungieKey(String);
fn grab_api_key() -> Result<BungieKey, Box<dyn std::error::Error>> {
    let api_key_path = "secrets/api.key";
    let contents = fs::read_to_string(api_key_path)?;
    Ok(BungieKey(contents))
}
