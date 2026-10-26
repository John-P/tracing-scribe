// © Copyright 2026 Helsing GmbH. All rights reserved.
//! The "Error Handling" snippet from the README, rendered into
//! `.images/error_handling.svg`.
use std::error::Error;
use std::fmt;

use tracing::info_span;
use tracing_scribe::console;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::Registry;

#[derive(Debug)]
struct MyError;

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "something went wrong")
    }
}

#[tracing::instrument(err)]
fn failing_task() -> Result<(), MyError> {
    console!(error, "this task is about to fail");
    Err(MyError)
}

fn main() {
    let subscriber = Registry::default().with(tracing_scribe::ConsoleLayer::default());
    tracing::subscriber::with_default(subscriber, || {
        let _span = info_span!("main").entered();
        failing_task().ok();
    });
}
