mod mascot;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    mascot::print();

    rs_service::startup().await
}
