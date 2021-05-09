use log::Level;

fn main() {
    dlog_rs::Builder::new()
        .with_env_api_key("DLOG_API_KEY")
        .with_level(Level::Info)
        .build();

    log::info!("Hello, world!");
    log::logger().flush();
}
