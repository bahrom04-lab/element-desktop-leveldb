use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path of to be parsed leveldb
    #[arg(short, long)]
    pub leveldb_path: String,
}
