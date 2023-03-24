/// This is a bare-bones example of the reqwest library.
async fn basic_get(path: &str) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    let response = reqwest::get(path).await?;
    let status = response.status();
    let text = response.text().await?;

    println!("{:#?}", text);

    Ok(status)
}
