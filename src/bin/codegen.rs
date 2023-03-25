pub mod models;

use crate::models::gpt_response::GptResponse;
use dotenvy::dotenv;
use serde_json::{from_reader, from_str, json};
use std::{
    env,
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
};

//PS Remove dir + children > rm D:\Generated_Repo -Force -Recurse
fn main() -> Result<(), reqwest::Error> {
    dotenv().expect(".env file not found");
    //REQUEST INITIAL CODE GENERATION BASED ON PROMPT
    let client = reqwest::blocking::Client::new();

    //Prod
    // let response: reqwest::blocking::Response = open_api_web_request(client)?;
    // match from_str::<GptResponse>(&response.text().unwrap()) {
    //     Ok(resp) => {
    //         _ = init_repo(resp);
    //     }
    //     Err(_) => eprint!("Error Parsing JSON Response..."),
    // }

    //Dev /Test
    let file = File::open("turbo3.5_response.json").expect("This must be here");
    let reader = BufReader::new(file);

    match from_reader::<_, GptResponse>(reader) {
        Ok(resp) => {
            _ = init_repo(resp, client);
        }
        Err(_) => eprint!("Error Parsing JSON Response..."),
    }

    Ok(())
}

fn open_api_web_request(
    client: reqwest::blocking::Client,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
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

    let response: reqwest::blocking::Response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(body)
        .send()?;
    // .text()?;
    Ok(response)
}

fn init_repo(resp: GptResponse, client: reqwest::blocking::Client) -> Result<(), std::io::Error> {
    let code_lines = &resp.choices[0]
        .message
        .content
        .lines()
        .map(|x| x)
        //Skip MD code init lines
        .filter(|c| !c.starts_with("```") && !c.is_empty())
        .collect::<Vec<&str>>();
    //TODO > Some api responses could have additional code explanations beside  ``` code ``` so i need to handle those too

    let base_dir = "D:\\generated_repo";
    fs::create_dir(base_dir)?;

    _ = std::process::Command::new("cargo")
        .args(&["init", base_dir])
        .output()
        .expect(format!("Failed to initialize project at {base_dir}").as_str());

    let file = File::create(format!("{base_dir}/src/main.rs")).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    code_lines.iter().enumerate().for_each(|(i, ln)| {
        writeln!(&mut writer, "{ln}")
            .expect(format!("File Writer FAILED AT Line:: {i} ,VAL:: {ln}").as_str())
    });

    writer.flush().expect("Failed to flush Writer");

    let success = check_build_err(base_dir);

    //Refetch OPEN AI api with broken code to try and resolve it
    if !success {
        search_code_fix(client).expect("I was supposed to get the response");
    }

    Ok(())
}

fn search_code_fix(client: reqwest::blocking::Client) -> Result<(), reqwest::Error> {
    let api_key = env::var("OPEN_AI_KEY").expect("OpenAI key requred!");
    let temperature = 0.2;
    let model = "gpt-3.5-turbo";

    let rust_code = r#"use std::io
    fn main() {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                print!("{}", input);
            }
            Err(error) => println!("Error: {}", error),
        }
    }"#;

    let content = format!(
        r#"{{could you fix this rust code to compile
            ```rust
            {rust_code}}}
            ```"#
    );

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

    println!("{body}\n");

    // let response = client
    //     .post("https://api.openai.com/v1/chat/completions")
    //     .header("Content-Type", "application/json")
    //     .header("Authorization", format!("Bearer {}", api_key))
    //     .body(body)
    //     .send()?
    //     .text()?;

    // println!("{response}");

    Ok(())

    //NOTE: we should retry UP TO 3 times with each new codeGEN /  stop spamming the api then
    //Add logging middleware to ouptut each iterations request reponses sequentially [Gen 1 (Request prompt ..., Response ...), Gen2.., Gen 3..]
}

///Check build errors in specific dir
///->
///Returns 'false' if there are build error's
fn check_build_err(base_dir: &str) -> bool {
    let output = std::process::Command::new("cargo")
        .args(&["check"])
        .current_dir(base_dir)
        .output()
        .expect("Failed to build project");

    if !output.status.success() {
        println!(
            "cargo check stderr: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
        return false;
    }
    println!("cargo check Succeeded!");
    //Optionally build app ? (after check has no ERRORs , "build --release")
    return true;
}

//TODO:
//1 create new dir in D:/NEW_REPO
//2 run cargo init <name_project>
//3 alter main.rs code or delete + create new one
//4 try building the app
//if errors are present parse them and pass to gpt api

//optionsls
//Generate folder structure with some cargo tools
//run some other cli or analyzers on top of the code on v2

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
