use actix_web::{post, web, Responder};
use rs_primitives::request::TxRequest;

#[post("/tx")]
async fn handle_tx(
    request: web::Json<TxRequest>,
    dispatcher: web::Data<rs_dispatcher::Dispatcher>,
) -> impl Responder {
    println!(
        "Received tx: {:?} on chain: {:?}",
        request.tx_hash, request.chain
    );

    let data = dispatcher.dispatch_onchain(request.into_inner()).await;
    web::Json(data)
}
