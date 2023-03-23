use std::env;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct GPTResponse {
    choices: Vec<GPTChoice>,
}

#[derive(Debug, Deserialize)]
struct GPTChoice {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct GptRequest {
    model: String,
    age: u8,
    phones: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");
    let api_key = env::var("OPEN_AI_KEY").expect("OpenAI key requred!");
    let temperature = 0.2;
    //let max_tokens = 10;
    let model = "gpt-3.5-turbo";
    let content ="generate Rust code for simple CLI app that takes input and prints it out, skip code explanation step.";

    let body = json!({
        "model": model,
        "messages":[
            {
                "role": "user", "content":content
            }
        ],
        "temperature": temperature
    })
    .to_string();

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)
        .send()?
        .text()?;

    //let gpt_response: GPTResponse = serde_json::from_str(&response)?;
    //println!("{:#?}", gpt_response);

    Ok(())
}

//https://github.com/f/awesome-chatgpt-prompts (idea for prompts )
//Readme docs >   https://markmap.js.org/repl

/* TESTING PROMPT
   generate Rust code for simple CLI app that takes input and prints it out, skip code explanation step.

   Example Request:
-------------------------------------------------
   curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
     "model": "gpt-3.5-turbo",
     "messages": [{"role": "user", "content": "generate Rust code for simple CLI app that takes input and prints it out, skip code explanation step."}],
     "temperature": 0.7
   }'
   -------------------------------------------------
   MODELS:
    gpt-3.5-turbo > https://platform.openai.com/docs/models/gpt-3-5
    code-davinci-002 >  Optimized for code-completion tasks | 8,001 tokens
   For windows replace '\' with '^' for newline or run in ubuntu console

*/

//Starter template which can be extended / updated acording to Open A's apis
//https://platform.openai.com/docs/api-reference/chat/create
//https://platform.openai.com/docs/api-reference/chat
//https://platform.openai.com/docs/models/model-endpoint-compatibility

//might help
//https://github.com/gencay/vscode-chatgpt