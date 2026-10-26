// © Copyright 2026 Helsing GmbH. All rights reserved.
//! The "Attaching Extra Information" snippet from the README, rendered into
//! `.images/extra_info.svg`.
use tracing::info_span;
use tracing_scribe::console;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::Registry;

#[tracing::instrument(fields(extra = "some extra info"))]
fn task_with_extra() {
    console!(info, "this task has extra information");
}

fn main() {
    let subscriber = Registry::default().with(tracing_scribe::ConsoleLayer::default());
    tracing::subscriber::with_default(subscriber, || {
        let _span = info_span!("main").entered();
        task_with_extra();
    });
}
