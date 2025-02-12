use rust_app_template::{config::AppConfig, error::AppResult};

fn main() -> AppResult<()> {
    let config = AppConfig::get();
    println!(
        "Hello, {} ({})",
        &config.distribution.name, &config.distribution.version
    );

    Ok(())
}
