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

async fn call_chat_gpt_api(question: &str) -> Result<String, String> {
    let api_key = "API";
    let client = Client::new();
    let payload = json!({
        "model": "gpt-3.5-turbo", // モデル名を追加
        "messages": [
            {"role": "system", "content": "You are a helpful assistant."},
            {"role": "assistant", "content": ""},
            {"role": "user", "content": question},
        ],
        "max_tokens": 100
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // 整形されたJSON文字列
    let pretty_str = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", pretty_str);

    Ok(response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

async fn chat_api(info: web::Json<ChatQuestion>) -> impl Responder {
    println!("input: {}", info.question);

    let chat_gpt_response = match call_chat_gpt_api(&info.question).await {
        Ok(response) => {
            println!("value: {}", response);
            response
        }
        Err(e) => {
            println!("{}", e.to_string());
            "An error occurred".to_string()
        }
    };

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
