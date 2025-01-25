mod handlers;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/heartbeat")]
async fn heartbeat() -> impl Responder {
    web::Json(serde_json::json!({
        "message": "Hi Admin, I am The Machin!"
    }))
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
