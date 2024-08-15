use std::env;
use std::error::Error;
use std::io::{self, Write};
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Define structs to match the API request and response
#[derive(Serialize)]
struct CodeCompletionRequest {
    messages: Vec<Message>,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct ResponsePayload {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

async fn send_api_request(request: &CodeCompletionRequest) -> Result<ResponsePayload, Box<dyn Error>> {
    dotenv().ok();
    let api_endpoint = env::var("API_ENDPOINT")?;
    let api_key = env::var("API_KEY")?;

    let client = Client::new();

    let response = client
        .post(&api_endpoint)
        .header("api-key", &api_key) // Use the api-key header
        .json(request)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;
    println!("Response status: {}", status);
    println!("Raw API response: {}", response_text);

    if status.is_success() {
        // Deserialize based on the actual response structure
        let parsed_response: ResponsePayload = serde_json::from_str(&response_text)?;
        Ok(parsed_response)
    } else {
        // Handle error responses
        Err(format!("API error: {} - {}", status.as_u16(), response_text).into())
    }
}

// Main function with async runtime
#[tokio::main]
async fn main() {
    loop {
        println!("AI Code Assistant");
        println!("1. Code Completion");
        println!("2. Code Explanation");
        println!("3. Refactoring Suggestions");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => code_completion().await,
            "2" => code_explanation().await,
            "3" => refactoring_suggestions().await,
            "4" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

// Feature functions
async fn code_completion() {
    println!("Enter your partial code (type 'END' on a new line when finished):");
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
    }

    let request = CodeCompletionRequest {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: code,
            },
        ],
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 100,
    };

    match send_api_request(&request).await {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                println!("Completion suggestion:");
                println!("{}", choice.message.content);
            } else {
                println!("No response generated.");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

async fn code_explanation() {
    println!("Enter the code snippet you want explained:");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let request = CodeCompletionRequest {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: format!("Explain the following code: {}", code.trim()),
            },
        ],
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 150,
    };

    match send_api_request(&request).await {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                println!("Explanation:");
                println!("{}", choice.message.content);
            } else {
                println!("No response generated.");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

async fn refactoring_suggestions() {
    println!("Enter the code block for refactoring suggestions:");
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let request = CodeCompletionRequest {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: format!("Suggest refactoring for the following code: {}", code.trim()),
            },
        ],
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 150,
    };

    match send_api_request(&request).await {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                println!("Original Code: {}", code.trim());
                println!("Suggested Refactor:");
                println!("{}", choice.message.content);
            } else {
                println!("No response generated.");
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
