use log::Level;

const SLEEP: std::time::Duration = std::time::Duration::from_micros(50);

#[tokio::main]
async fn main() {
    dlog::Builder::new()
        .with_env_api_key("DLOG_API_KEY")
        .with_level(Level::Debug)
        .with_email_sanitizer()
        .with_credit_card_sanitizer()
        .build();

    for i in 0..1_000_000 {
        log::debug!("Starting iteration: {}", i);
        log::info!(
            "Tracing timestamp: {}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        log::debug!("Ending iteration: {}", i);

        tokio::time::sleep(SLEEP).await;
    }

    log::logger().flush();
}
