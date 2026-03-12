use anyhow::Result;
use clap::Parser;
use element_metadata::{ElementLevelDBParser, ElementMetadata};
use element_metadata_cli::Args;
use std::path::Path;

fn main() -> Result<()> {
    println!("Element Desktop LevelDB Metadata Parser");
    println!("========================================\n");

    let args = Args::parse();

    // Example usage - user would provide their Element LevelDB path
    let example_path = args.leveldb_path;

    println!("To use this parser: {example_path}");
    println!("1. Locate your Element LevelDB database:");
    println!("   - Windows: %APPDATA%\\Element\\Local Storage\\leveldb");
    println!("   - Linux: ~/.config/Element/Local Storage/leveldb");
    println!("   - macOS: ~/Library/Application Support/Element/Local Storage/leveldb");
    println!("\n2. Provide the path to the parser\n");

    // Check if example path exists
    if Path::new(&example_path).exists() {
        match ElementLevelDBParser::open(example_path) {
            Ok(parser) => {
                println!("✓ Successfully opened LevelDB database");

                match parser.to_json() {
                    Ok(json) => {
                        println!("\nExtracted Metadata (JSON):");
                        println!("{}", json);
                    }
                    Err(e) => eprintln!("Error parsing metadata: {}", e),
                }
            }
            Err(e) => eprintln!("Error opening database: {}", e),
        }
    }
    Ok(())
}
