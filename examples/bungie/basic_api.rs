
/// A rudementary test funciton of this for verification
/// #[tokio::test]
///  async fn bungie_manifest() {
///     let client = _make_client();
///     // Gjallarhorn!
///     let response = bungie_no_auth(&client, "/Destiny/Manifest/InventoryItem/1274330687/")
///         .await
///         .unwrap();
///     assert_eq!(reqwest::StatusCode::OK, response);
///  }
async fn bungie_no_auth(
    client: &Client,
    path: &str,
) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    println!("going to: {}", BUNGIE_ROOT.to_owned() + path);

    let response = client.get(BUNGIE_ROOT.to_owned() + path).send().await?;
    let status = response.status();

    println!("{}", response.text().await?);

    Ok(status)
}