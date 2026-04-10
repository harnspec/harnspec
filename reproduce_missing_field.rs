use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:3000/api/sessions")
        .json(&json!({
            "specIds": ["001-test"],
            "runner": "claude",
            "mode": "autonomous"
        }))
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Body: {}", res.text().await?);

    Ok(())
}
