use std::path::PathBuf;
use tracing::{error, info, warn, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn setup(log_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&log_dir)?;

    // Setup file appender - will create daily rotating logs
    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "app.log");

    // Create a custom time formatter
    let timer = UtcTime::rfc_3339();

    // Setup formatting for file logging
    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_timer(timer.clone())
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true);

    // Setup console logging
    let console_layer = fmt::layer()
        .with_timer(timer)
        .with_thread_ids(true)
        .with_target(true);

    // Setup filtering - you can override with RUST_LOG environment variable
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Combine layers and install the subscriber
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(file_layer)
        .with(console_layer)
        .init();

    Ok(())
}
