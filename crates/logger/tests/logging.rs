#![cfg(feature = "host")]

use tracing_subscriber::fmt;

#[test]
fn logging_macros_emit_events() {
    let subscriber = fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        logger::debug!("Debug message: {}", 42);
        logger::info!("Info message: {}", 42);
        logger::warn!("Warning message: {}", 42);
        logger::error!("Error message: {}", 42);
    });
}
