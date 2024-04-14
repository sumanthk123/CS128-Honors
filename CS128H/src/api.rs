use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize)]
struct ApiRequest {
    question: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    answer: String,
    explanation: String,
}

async fn fetch_answer(client: &Client, question: &str, api_url: &str) -> Result<ApiResponse, reqwest::Error> {
    let request_body = ApiRequest {
        question: question.to_string(),
    };

    let response = client
        .post(api_url)
        .json(&request_body)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let api_url = "http://example.com/api"; // Replace with the actual API URL
    let question = "What is the capital of France?";

    match fetch_answer(&client, question, api_url).await {
        Ok(response) => {
            println!("Answer: {}", response.answer);
            println!("Explanation: {}", response.explanation);
        }
        Err(e) => {
            println!("Error fetching answer: {}", e);
        }
    }
}
