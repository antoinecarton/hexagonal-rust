use std::net::TcpListener;

use infra::settings::Settings;
use infra::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::load().expect("Failed to load configuration.");
    let address = format!("127.0.0.1:{}", settings.web.port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;
    Ok(())
}