use tracing::info;
use tracing_subscriber::filter::{LevelFilter, filter_fn};
use tracing_subscriber::{Layer, prelude::*};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::filter::FilterExt;


fn main() {
    let level_filter = LevelFilter::INFO;
    let target_filter = filter_fn(|meta| {
        meta.target().starts_with("interesting_target")
    });
    let filter = target_filter.and(level_filter);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .init();

    // This event will *not* be enabled:
    tracing::info!("an event with an uninteresting target");

    // This event *will* be enabled:
    tracing::info!(target: "interesting_target", "a very interesting event");

    // This event will *not* be enabled:
    tracing::debug!(target: "interesting_target", "interesting debug event...");
}
