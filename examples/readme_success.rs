// © Copyright 2026 Helsing GmbH. All rights reserved.
//! The "Example" snippet from the README, rendered into `.images/success.svg`.
use tracing::info_span;
use tracing_scribe::console;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::Registry;

#[tracing::instrument]
fn outer_task() {
    inner_task();
}

#[tracing::instrument]
fn inner_task() {
    console!(notice, "generating unicorns");
    std::thread::sleep(std::time::Duration::from_millis(100));
    console!(info, "unicorns generation completed");
}

fn main() {
    let subscriber = Registry::default().with(tracing_scribe::ConsoleLayer::default());
    tracing::subscriber::with_default(subscriber, || {
        let _span = info_span!("main").entered();
        outer_task();
    });
}
