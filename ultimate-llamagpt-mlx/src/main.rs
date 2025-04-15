//! Ultimate LlamaGPT-MLX: Main entry point.
mod cli;
mod config;
mod commands;
mod tui;
mod ml;
mod themes;
mod utils;
mod cache;
mod chat;
mod integration;
mod telemetry;
mod animation;

use clap::Parser;
use tracing::{info, error};
use std::error::Error;
use cli::Cli;
use config::Config;
use commands::registry::CommandRegistry;
use telemetry::setup_telemetry;
use tui::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load().unwrap_or_default();
    let _guard = setup_telemetry(&config);
    info!("🦙 Starting Ultimate LlamaGPT-MLX v{}", env!("CARGO_PKG_VERSION"));
    let args = Cli::parse();
    let command_registry = CommandRegistry::new();
    if args.one_shot_mode() {
        return cli::handle_one_shot(&args, &config, &command_registry).await;
    }
    if args.chat_mode() {
        return cli::handle_chat(&args, &config, &command_registry).await;
    }
    match tui::app::run_app(App::new(&config, command_registry)) {
        Ok(_) => { info!("Application terminated successfully"); Ok(()) },
        Err(err) => { error!("Application error: {}", err); Err(Box::new(err)) }
    }
}
