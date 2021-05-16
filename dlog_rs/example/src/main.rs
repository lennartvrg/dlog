use log::Level;

#[tokio::main]
async fn main() {
    dlog_rs::Builder::new()
        .with_env_api_key("DLOG_API_KEY")
        .with_level(Level::Info)
        .with_email_sanitizer()
        .with_credit_card_sanitizer()
        .build();

    log::info!("Testing");

    log::logger().flush();
}
