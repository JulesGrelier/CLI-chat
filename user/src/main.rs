use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client = reqwest::Client::new();

    let response = client
        .post("http://127.0.0.1:8000")
        .header("Content-Type", "text/plain")
        .body("Hello world")
        .send()
        .await?;

    println!("Response : {:?}", response);
    println!("Body : {}", response.text().await?);

    Ok(())
}