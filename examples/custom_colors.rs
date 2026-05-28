// © Copyright 2026 Helsing GmbH. All rights reserved.
use owo_colors::colors;
use std::time::Duration;
use tracing::info_span;
use tracing_scribe::{ColorScheme, ConsoleLayer, console};
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::Registry;

#[tracing::instrument]
fn database_operations() {
    console!(info, "connecting to database");
    std::thread::sleep(Duration::from_millis(100));
    console!(info, "connection established");

    console!(notice, "running migrations");
    std::thread::sleep(Duration::from_millis(150));
    console!(info, "migrations completed");
}

#[tracing::instrument]
fn api_calls() {
    console!(info, "fetching user data");
    std::thread::sleep(Duration::from_millis(80));
    console!(info, "user data retrieved");

    console!(warn, "rate limit approaching");
    std::thread::sleep(Duration::from_millis(50));

    console!(error, "api request failed: timeout");
}

#[tracing::instrument]
fn processing_task() {
    console!(notice, "starting data processing");
    database_operations();
    api_calls();
    console!(info, "processing complete");
}

fn main() {
    println!("=== Example with Custom Colors ===\n");

    // Create a custom color scheme with bright colors
    let colors = ColorScheme::default()
        .with_success_color(colors::Green)
        .with_error_color(colors::Red)
        .with_warning_color(colors::Yellow)
        .with_notice_color(colors::Magenta)
        .with_tree_color(colors::Magenta)
        .with_extra_color(colors::BrightWhite);

    let layer = ConsoleLayer::default().with_colors(colors);
    let subscriber = Registry::default().with(layer);

    tracing::subscriber::with_default(subscriber, || {
        let _span = info_span!("application", extra = "v1.0.0").entered();
        console!(notice, "application started");
        processing_task();
        console!(info, "application finished");
    });

    println!("\n=== Try modifying the colors above to see different styles! ===");
    println!("Available colors: BrightGreen, BrightRed, BrightYellow, BrightMagenta,");
    println!("                  BrightCyan, BrightBlue, Green, Red, Yellow, Magenta, Cyan, Blue");
}
