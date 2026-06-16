mod api;
mod common;
mod database;
mod messages;
mod models;
mod resources;

use clap::{Parser, Subcommand};
use common::{config::Config, context::Context};
use models::tts;
use std::fs;

#[derive(Parser)]
#[command(name = "llaas")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Book {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
    },
    Tts {
        #[arg(long)]
        text: String,
        #[arg(long)]
        file: String,
        #[arg(long, default_value = "en")]
        lang: String,
    },
    Video {
        #[arg(long)]
        url: String,
        #[arg(long, default_value = "en")]
        languages: Vec<String>,
    },
    Start {
        #[arg(long, default_value = "8080")]
        port: u16,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize the application context with configuration settings.
    // The context is created as a static reference to ensure it lives for the entire duration of the application
    // and can be safely shared across threads and async tasks without needing to clone or manage lifetimes manually.
    let context: &'static Context = Box::leak(Box::new(Context::new(Config::new())));

    match cli.command {
        Commands::Book { from, to } => {
            let book = resources::epub::read(&from).expect("Failed to read epub");
            let json = book_to_json(&book);
            let output = serde_json::to_string_pretty(&json).expect("Failed to serialize JSON");
            fs::write(&to, output).expect("Failed to write output file");
            println!("Written to {to}");
        }
        Commands::Tts { text, file, lang } => {
            tts::save_as_wav(tts::TtsPreset::OmniVoice, &text, &file, &lang).unwrap();
            println!("Written to {file}");
        }
        Commands::Start { port } => {
            println!("Starting server on port {port}...");
            if let Err(err) = api::server::start_server(context, port).await {
                eprintln!("Server failed to start: {err}");
            }
        }
        Commands::Video { url, languages } => {
            let result = resources::video::download(
                context,
                &url,
                &languages.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            )
            .await
            .unwrap();
            println!("Downloaded video from URL: {}", result.url);
        }
    }
}

fn book_to_json(book: &messages::Book) -> serde_json::Value {
    serde_json::json!({
        "title": book.title,
        "author": book.author,
        "description": book.description,
        "keywords": book.keywords,
        "chapters": book.chapters.iter().map(|c| serde_json::json!({
            "title": c.title,
            "paragraphs": c.paragraphs.iter().map(|p| serde_json::json!({
                "lines": p.lines.iter().map(|l| serde_json::json!({ "text": l.text })).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    })
}
