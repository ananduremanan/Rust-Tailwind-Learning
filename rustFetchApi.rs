use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://raw.githubusercontent.com/ananduremanan/tempDirectory/main/tempData.json")
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Process the JSON response
    println!("{:?}", response);

    Ok(())
}
