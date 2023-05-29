use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "YOUR_API_KEY" with your MSG91 API key
    let api_key = "YOUR_API_KEY";
    let endpoint = "https://api.msg91.com/api/v2/sendsms";

    // Compose the request payload
    let mut payload = HashMap::new();
    payload.insert("sender", "SENDER_NAME");
    payload.insert("route", "4"); // 4 is the default promotional route
    payload.insert("country", "91"); // Country code (e.g., 91 for India)
    payload.insert("sms", "Hello, world!"); // The message content
    payload.insert("mobiles", "PHONE_NUMBER"); // The recipient's phone number

    // Create a new reqwest Client
    let client = Client::new();

    // Send the POST request to the MSG91 API
    let response = client
        .post(endpoint)
        .query(&[("authkey", api_key)])
        .json(&payload)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        println!("Message sent successfully!");
    } else {
        println!("Failed to send the message. Status: {}", response.status());
    }

    Ok(())
}
