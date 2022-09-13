use log::Level;

#[tokio::main]
async fn main() {
    dlog_rs::Builder::new()
        .with_str_api_key("cd86f604-234f-47a2-9dc0-b6f150ccf7fd")
        .with_level(Level::Debug)
        .with_email_sanitizer()
        .with_credit_card_sanitizer()
        .build();

    log::info!("Foo Bar");

    log::logger().flush();
}
