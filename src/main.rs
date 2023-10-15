use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct ChatQuestion {
    question: String,
}

#[derive(Serialize)]
struct ChatAnswer {
    answer: String,
}

async fn call_chat_gpt_api(question: &str) -> Result<String, reqwest::Error> {
    let api_key = "API_KEY";
    let client = Client::new();
    let payload = json!({
        "prompt": question,
        "max_tokens": 10
    });

    let response = client
        .post("https://api.openai.com/v1/engines/davinci-codex/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response["choices"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

async fn chat_api(info: web::Json<ChatQuestion>) -> impl Responder {
    println!("2{}", info.question);
    let chat_gpt_response = call_chat_gpt_api(&info.question)
        .await
        .unwrap_or("An error occurred".to_string());
    println!("res{}", chat_gpt_response);
    HttpResponse::Ok().json(ChatAnswer {
        answer: chat_gpt_response,
    })
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("konntiha");
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(hello)
            .route("/api/chat", web::post().to(chat_api))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
