use log::Level;

fn main() {
    dlog_rs::Builder::new()
        .with_env_api_key("DLOG_API_KEY")
        .with_level(Level::Info)
        .with_email_sanitizer()
        .with_credit_card_sanitizer()
        .build();

    log::info!("Hello, world 4000000760000002");
    log::logger().flush();
}
