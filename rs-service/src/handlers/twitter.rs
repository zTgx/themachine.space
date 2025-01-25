use actix_web::{post, web, Responder};
use rs_primitives::request::XRequest;

#[post("/x")]
async fn handle_x(request: web::Json<XRequest>) -> impl Responder {
    let XRequest { link } = request.into_inner();
    println!("Received link: {:?} on X", link);

    web::Json(serde_json::json!({
        "message": format!("Hi Admin, I am The Machin! on {}", link),
    }))
}
