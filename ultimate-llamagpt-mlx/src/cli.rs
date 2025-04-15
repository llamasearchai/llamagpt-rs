//! CLI argument definitions and handling logic.
use std::error::Error;
use clap::Parser;
use colored::Colorize;
use tracing::{info, warn};
use crate::commands::registry::CommandRegistry;
use crate::config::Config;
use crate::chat::ChatSession;

#[derive(Debug, Parser)]
#[command(
    name = "llamagpt",
    about = "Ultimate LlamaGPT-MLX: Advanced intelligent CLI assistant with on-device inference",
    version,
    author,
    long_about = "A fully-featured intelligent CLI assistant with TUI interface, on-device inference via MLX, and Llama-themed experience."
)]
pub struct Cli {
    #[arg(default_value = "")]
    pub prompt: String,
    #[arg(short, long)]
    pub chat: Option<String>,
    #[arg(short, long)]
    pub headless: bool,
    #[arg(long)]
    pub no_animations: bool,
    #[arg(short, long, default_value = "llama3-8b-q4")]
    pub model: String,
    #[arg(short, long)]
    pub voice: bool,
    #[arg(short, long, default_value = "0.7")]
    pub temperature: f32,
    #[arg(short, long)]
    pub debug: bool,
}

impl Cli {
    pub fn one_shot_mode(&self) -> bool {
        !self.prompt.is_empty() && self.chat.is_none()
    }
    pub fn chat_mode(&self) -> bool {
        self.chat.is_some()
    }
}

pub async fn handle_one_shot(args: &Cli, config: &Config, registry: &CommandRegistry) -> Result<(), Box<dyn Error>> {
    let prompt = &args.prompt;
    info!("Processing one-shot command: {}", prompt);
    if let Some(command) = registry.find(prompt) {
        match command.execute("{}").await {
            Ok(output) => { println!("{}", output); Ok(()) },
            Err(e) => { eprintln!("{} {}", "Error:".red().bold(), e); Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))) }
        }
    } else {
        match crate::ml::generate_response(prompt, args.temperature, &args.model).await {
            Ok(response) => { println!("{}", response); Ok(()) },
            Err(e) => { eprintln!("{} {}", "AI Error:".red().bold(), e); Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e))) }
        }
    }
}

pub async fn handle_chat(args: &Cli, config: &Config, registry: &CommandRegistry) -> Result<(), Box<dyn Error>> {
    let chat_id = args.chat.as_ref().unwrap();
    info!("Starting chat session: {}", chat_id);
    let mut session = ChatSession::new(config, chat_id);
    session.load_history();
    if !args.prompt.is_empty() {
        session.add_message("user", &args.prompt);
        let response = if let Some(command) = registry.find(&args.prompt) {
            command.execute("{}").await.unwrap_or_else(|e| e)
        } else {
            crate::ml::generate_response(&args.prompt, args.temperature, &args.model).await.unwrap_or_else(|e| e)
        };
        session.add_message("assistant", &response);
        println!("{} {}", "🦙".cyan(), response);
        session.save_history()?;
        return Ok(());
    }
    let mut rl = rustyline::Editor::<()>::new()?;
    println!("{} Welcome to Ultimate LlamaGPT-MLX Chat! Type 'exit' or 'quit' to end.", "🦙".cyan());
    if !session.history().messages.is_empty() {
        println!("{}", "=== Chat History ===".cyan().bold());
        for msg in &session.history().messages {
            let prefix = if msg.role == "user" { "You: ".blue() } else { "🦙 ".cyan() };
            println!("{}{}", prefix, msg.content);
        }
        println!("{}", "===================".cyan().bold());
    }
    loop {
        let prompt = format!("{} ", "🦙>".cyan().bold());
        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() { continue; }
                if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
                    println!("{}", "Goodbye! 👋".cyan());
                    break;
                }
                rl.add_history_entry(input);
                session.add_message("user", input);
                use std::io::{stdout, Write};
                use std::time::Duration;
                if !args.no_animations {
                    let thinking = ["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"];
                    print!("{} Thinking ", "🦙".cyan());
                    stdout().flush()?;
                    for i in 0..15 {
                        print!("{}", thinking[i % thinking.len()]);
                        stdout().flush()?;
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        print!("\x08");
                    }
                    println!("         ");
                }
                let response = if let Some(command) = registry.find(input) {
                    command.execute("{}").await.unwrap_or_else(|e| e)
                } else {
                    crate::ml::generate_response(input, args.temperature, &args.model).await.unwrap_or_else(|e| e)
                };
                session.add_message("assistant", &response);
                println!("{} {}", "🦙".cyan(), response);
                session.save_history()?;
            },
            Err(rustyline::error::ReadlineError::Interrupted) => { println!("Ctrl-C pressed, exiting..."); break; },
            Err(rustyline::error::ReadlineError::Eof) => { println!("Ctrl-D pressed, exiting..."); break; },
            Err(err) => { warn!("Error reading input: {}", err); break; }
        }
    }
    Ok(())
}
