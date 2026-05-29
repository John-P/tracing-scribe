// © Copyright 2026 Helsing GmbH. All rights reserved.
use owo_colors::OwoColorize;
use std::time::Duration;
use tracing::{Instrument, info_span, subscriber};
use tracing_scribe::{ConsoleLayer, console};
use tracing_subscriber::{layer::SubscriberExt, registry::Registry};

#[tracing::instrument]
fn configuration() {
    console!(info, "project root folder: {}", "/home/user/project".blue());
    console!(
        info,
        "configuration loaded from {}",
        "/home/user/project/config.toml".blue()
    );
    std::thread::sleep(Duration::from_millis(1000));
}

#[tracing::instrument]
fn load_environment() {
    console!(info, "git version: {}", "0.0.0-41-gba29802".purple());
    console!(info, "git rev short: {}", "ba29802".purple());
    console!(info, "semver version: {}", "0.0.0".blue());
    console!(info, "release version: {}", "0.0.0".yellow());
    console!(
        error,
        "ci commit tag: {}",
        "CI_COMMIT_TAG missing from environment".red()
    );
    console!(
        error,
        "ci commit branch: {}",
        "CI_COMMIT_BRANCH missing from environment".red()
    );
    console!(
        error,
        "ci project name: {}",
        "CI_PROJECT_NAME missing from environment".red()
    );
}

#[tracing::instrument]
async fn push_data() {
    console!(info, "pushing crate {}", "tracing-scribe".red());
    tokio::time::sleep(Duration::from_millis(10)).await;
    console!(
        info,
        "crate {} published successfully",
        "tracing-scribe".red()
    );
}

#[tracing::instrument]
async fn publish() {
    configuration();
    load_environment();
    push_data().instrument(tracing::Span::current()).await;
}

#[tracing::instrument(name = "run command git checkout -b bump/v0.15.4", err)]
fn git_checkout_new_branch_exists() -> Result<(), String> {
    console!(warn, "fatal: a branch named 'bump/v0.15.4' already exists");
    std::thread::sleep(Duration::from_millis(2));
    Err("error running command git checkout -b bump/v0.15.4, exit code: Some(128)".to_string())
}

#[tracing::instrument(name = "bump", err)]
fn bump_branch_exists_inner() -> Result<(), String> {
    {
        let _span = tracing::info_span!("configure git").entered();
        {
            let _cmd = tracing::info_span!("run command git config user.name Test User").entered();
        }
        {
            let _cmd =
                tracing::info_span!("run command git config user.email test@example.com").entered();
        }
        {
            let _cmd = tracing::info_span!(
                "run command git remote set-url origin https://git.example.com/example/project.git"
            )
            .entered();
        }
    }
    {
        let _span = tracing::info_span!("fetch and checkout").entered();
        {
            let _cmd = tracing::info_span!("run command git fetch origin develop").entered();
            console!(info, "from https://git.example.com/example/project");
            console!(info, "* branch            develop    -> FETCH_HEAD");
            std::thread::sleep(Duration::from_millis(672));
        }
        {
            let _cmd = tracing::info_span!("run command git checkout origin/develop").entered();
            console!(info, "head is now at 5671839 initial commit");
        }
    }
    console!(info, "current version: {}", "0.15.3".blue());
    console!(info, "new version: {}", "0.15.4".blue());
    {
        let _cmd = tracing::info_span!(
            "run command git ls-remote --exit-code --heads origin bump/v0.15.4"
        )
        .entered();
        std::thread::sleep(Duration::from_millis(293));
    }
    git_checkout_new_branch_exists()
}

#[tracing::instrument(name = "bump", err)]
fn bump_branch_exists_outer() -> Result<(), String> {
    bump_branch_exists_inner()
}

#[tracing::instrument(name = "run command git push -u origin HEAD", err)]
fn git_push_denied() -> Result<(), String> {
    console!(
        warn,
        "remote: you are not allowed to push code to this project."
    );
    console!(
        warn,
        "fatal: unable to access 'https://git.example.com/example/project.git/': the requested url returned error: 403"
    );
    std::thread::sleep(Duration::from_millis(104));
    Err("you are not allowed to push code to this project.".to_string())
}

#[tracing::instrument(name = "commit and push", err)]
fn commit_and_push() -> Result<(), String> {
    {
        let _cmd = tracing::info_span!("run command git add Cargo.toml Cargo.lock").entered();
    }
    {
        let _cmd =
            tracing::info_span!("run command git commit -m bump version to 0.15.4").entered();
        console!(info, "[bump/v0.15.4 f940544] bump version to 0.15.4");
        console!(info, "2 files changed, 2 insertions(+), 2 deletions(-)");
    }
    git_push_denied()
}

#[tracing::instrument(name = "bump", err)]
fn bump_push_denied_inner() -> Result<(), String> {
    {
        let _span = tracing::info_span!("configure git").entered();
        {
            let _cmd = tracing::info_span!("run command git config user.name Test User").entered();
        }
        {
            let _cmd =
                tracing::info_span!("run command git config user.email test@example.com").entered();
        }
        {
            let _cmd = tracing::info_span!(
                "run command git remote set-url origin https://git.example.com/example/project.git"
            )
            .entered();
        }
    }
    {
        let _span = tracing::info_span!("fetch and checkout").entered();
        {
            let _cmd = tracing::info_span!("run command git fetch origin develop").entered();
            console!(info, "from https://git.example.com/example/project");
            console!(info, "* branch            develop    -> FETCH_HEAD");
            std::thread::sleep(Duration::from_millis(387));
        }
        {
            let _cmd = tracing::info_span!("run command git checkout origin/develop").entered();
            console!(info, "head is now at 5671839 initial commit");
            std::thread::sleep(Duration::from_millis(214));
        }
    }
    console!(info, "current version: {}", "0.15.3".blue());
    console!(info, "new version: {}", "0.15.4".blue());
    {
        let _cmd = tracing::info_span!(
            "run command git ls-remote --exit-code --heads origin bump/v0.15.4"
        )
        .entered();
        std::thread::sleep(Duration::from_millis(411));
    }
    {
        let _cmd = tracing::info_span!("run command git checkout -b bump/v0.15.4").entered();
        console!(info, "switched to a new branch 'bump/v0.15.4'");
    }
    {
        let _span = tracing::info_span!("update cargo.toml").entered();
        console!(info, "updated Cargo.toml to version {}", "0.15.4".blue());
    }
    {
        let _span = tracing::info_span!("update cargo.lock").entered();
        {
            let _cmd = tracing::info_span!("run command cargo update --workspace").entered();
            console!(info, "locking 1 package to latest compatible version");
            console!(
                info,
                "updating example-project v0.15.3 (/builds/example/project) -> v0.15.4"
            );
            std::thread::sleep(Duration::from_millis(411));
        }
        console!(info, "updated Cargo.lock");
    }
    commit_and_push()
}

#[tracing::instrument(name = "bump", err)]
fn bump_push_denied_outer() -> Result<(), String> {
    bump_push_denied_inner()
}

fn main() {
    let subscriber = Registry::default().with(ConsoleLayer::default());

    println!("=== publish failure ===\n");
    subscriber::with_default(subscriber, || {
        let _span = info_span!("cli").entered();
        console!(
            notice,
            "run id: {}",
            "ab1c3944-d5c9-48c9-b3e8-39341d2f64f5".blue()
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(publish())
    });

    println!("\n\n=== bump failure: branch already exists ===\n");
    let subscriber = Registry::default().with(ConsoleLayer::default());
    subscriber::with_default(subscriber, || {
        let _cli = info_span!("cli").entered();
        console!(
            notice,
            "run id: {}",
            "019c7593-1f68-784a-ade1-9a06a33a38be".blue()
        );
        bump_branch_exists_outer().ok();
        console!(
            notice,
            "run id: {}",
            "019c7593-1f68-784a-ade1-9a06a33a38be".blue()
        );
    });

    println!("\n\n=== bump failure: push permission denied ===\n");
    let subscriber = Registry::default().with(ConsoleLayer::default());
    subscriber::with_default(subscriber, || {
        let _cli = info_span!("cli").entered();
        console!(
            notice,
            "run id: {}",
            "019c75a8-e7a6-7bec-822a-e371c363d73e".blue()
        );
        bump_push_denied_outer().ok();
        console!(
            notice,
            "run id: {}",
            "019c75a8-e7a6-7bec-822a-e371c363d73e".blue()
        );
    });
}
