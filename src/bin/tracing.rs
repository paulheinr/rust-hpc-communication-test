use std::io::stdout;
use std::{fs::File, thread::sleep, time::Duration};
use tracing::{info, instrument, trace};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, fmt::writer::BoxMakeWriter, EnvFilter, Layer};

#[instrument] // a span is created and entered for the whole function; args are recorded
fn outer(n: i32) {
    trace!("about to call `work`"); // lightweight event inside the current span
    let res = work(n, 15);
    info!(result = res, "done");
}

#[instrument(skip(delay_ms))] // record x, but don't record delay_ms
fn work(x: i32, delay_ms: u64) -> i32 {
    trace!("starting inner work");
    sleep(Duration::from_millis(delay_ms));
    let y = x * 2;
    trace!(?y, "after compute"); // ? uses Debug formatting for the field
    y
}

fn main() {
    // Minimal subscriber: human-readable formatting + env-based filter
    let subscriber = fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                // ensure we see `trace!` level by default if RUST_LOG isn't set
                .add_directive(tracing::Level::TRACE.into()),
        )
        .with_target(false)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    outer(21);
}
