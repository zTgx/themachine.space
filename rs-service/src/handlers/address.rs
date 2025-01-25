use actix_web::{post, web, Responder};
use rs_primitives::request::AddressRequest;

#[post("/address")]
async fn handle_address(
    request: web::Json<AddressRequest>,
    dispatcher: web::Data<rs_dispatcher::Dispatcher>,
) -> impl Responder {
    println!(
        "Received ca: {:?} on chain: {:?}",
        request.address, request.chain
    );

    let data = dispatcher.dispatch_onchain(request.into_inner()).await;
    web::Json(data)
}
