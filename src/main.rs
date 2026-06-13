mod models;
mod resources;
pub mod messages;

use clap::{Parser, Subcommand};
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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Book { from, to } => {
            let book = resources::epub::read(&from).expect("Failed to read epub");
            let json = book_to_json(&book);
            let output = serde_json::to_string_pretty(&json).expect("Failed to serialize JSON");
            fs::write(&to, output).expect("Failed to write output file");
            println!("Written to {to}");
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
