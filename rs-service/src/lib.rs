mod handlers;

use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use solagent::{
    goplus::solana_token_security_info,
    pyth_fetch_price::FetchPricePyTh,
    rig::{completion::Prompt, providers},
};

#[derive(Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct MessageRequest {
    messages: Vec<Message>,
}

#[get("/heartbeat")]
async fn heartbeat() -> impl Responder {
    web::Json(serde_json::json!({
        "message": "Hi Admin, I am The Machin!"
    }))
}

#[post("/chat")]
async fn chat(req_body: web::Json<MessageRequest>) -> String {
    // 1. Extract messages from the request body
    let messages = req_body.into_inner().messages;

    let mut prompt = "".to_string();
    // 2. Process the messages (example: print to console)
    for message in messages {
        println!("Role: {}, Content: {}", message.role, message.content);
        prompt = message.content;
    }

    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("llama3.2")
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
        )
        .tool(solana_token_security_info::SolanaTokenSecurityInfo)
        .tool(FetchPricePyTh)
        .build();

    // Prompt the agent and print the response
    let response_message = comedian_agent.prompt(&prompt).await.unwrap_or_default();

    // 3. Create a response message
    // let response_message = "Messages received successfully!".to_string();
    response_message
}

pub async fn startup() -> std::io::Result<()> {
    let dispatcher = rs_dispatcher::Dispatcher::new();
    let data = web::Data::new(dispatcher);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(heartbeat)
            .service(handlers::tx::handle_tx)
            .service(handlers::ca::handle_ca)
            .service(handlers::address::handle_address)
            .service(handlers::twitter::handle_x)
            .service(chat)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
