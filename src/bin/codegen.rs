//Starter template which can be extended / updated acording to Open A's apis 
//https://platform.openai.com/docs/api-reference/chat/create

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct GPTResponse {
    choices: Vec<GPTChoice>,
}

#[derive(Debug, Deserialize)]
struct GPTChoice {
    text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "YOUR_API_KEY_HERE";
    let prompt = "Hello, OpenAI!";
    let temperature = 0.5;
    let max_tokens = 10;

    let url = format!(
        "https://api.openai.com/v1/completions?model=davinci&prompt={}&temperature={}&max_tokens={}",
        prompt, temperature, max_tokens
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(format!(
            r#"{{"prompt": "{}", "temperature": {}, "max_tokens": {}}}"#,
            prompt, temperature, max_tokens
        ))
        .send()?
        .text()?;

    let gpt_response: GPTResponse = serde_json::from_str(&response)?;

    println!("{:#?}", gpt_response);

    Ok(())
}
