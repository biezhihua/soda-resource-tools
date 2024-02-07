#![allow(warnings)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;

use rocket::{Build, Rocket};

pub(crate) mod db;
pub(crate) mod api;
pub(crate) mod task;
pub(crate) mod config;
pub(crate) mod utils;
mod global;

#[get("/")]
fn index() -> &'static str {
    "Hello, soda resource tools!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    init_tracing();
    rocket::build()
        .attach(api::stage())
        .attach(db::stage())
        .attach(task::stage())
        .mount("/", routes![index])
}

/// 初始化日志配置
fn init_tracing() {
    use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};
    // Configure a `tracing` subscriber that logs traces emitted by the server.
    tracing_subscriber::fmt()
        // Filter what traces are displayed based on the RUST_LOG environment
        // variable.
        //
        // Traces emitted by the example code will always be displayed. You
        // can set `RUST_LOG=tokio=trace` to enable additional traces emitted by
        // Tokio itself.
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("soda=info".parse().unwrap())
            .add_directive("rocket=info".parse().unwrap())
        )
        // Log events when `tracing` spans are created, entered, exited, or
        // closed. When Tokio's internal tracing support is enabled (as
        // described above), this can be used to track the lifecycle of spawned
        // tasks on the Tokio runtime.
        .with_span_events(FmtSpan::FULL)
        // Set this subscriber as the default, to collect all traces emitted by
        // the program.
        .init();

    tracing::info!("soda server start");
}
