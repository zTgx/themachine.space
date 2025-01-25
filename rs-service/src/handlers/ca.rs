use actix_web::{post, web, Responder};
use rs_primitives::request::CaRequest;

#[post("/ca")]
async fn handle_ca(
    request: web::Json<CaRequest>,
    dispatcher: web::Data<rs_dispatcher::Dispatcher>,
) -> impl Responder {
    println!(
        "Received ca: {:?} on chain: {:?}",
        request.ca, request.chain
    );

    let data = dispatcher.dispatch_onchain(request.into_inner()).await;
    web::Json(data)
}
