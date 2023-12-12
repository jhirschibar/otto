use reqwest::header::HeaderValue;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://us.sms.api.sinch.com/xms/v1/{}/batches"; //add project id

    let headers = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers
    };

    let client = Client::new();

    let response = client
        .get(url)
        .headers(headers)
        .header("Authorization", "Bearer ") // add bearer token
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", response);

    Ok(())
}

    