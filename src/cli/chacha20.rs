use std::path::PathBuf;

use clap::Parser;

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
pub enum ChaCha20SubCommand {
    #[command(name = "encrypt", about = "Encrypt with ChaCha20")]
    Encrypt(ChaCha20Opts),
    #[command(name = "decrypt", about = "Decrypt with ChaCha20")]
    Decrypt(ChaCha20Opts),
    #[command(name = "generate", about = "Generate key")]
    Generate(ChaCha20GenerateOpts),
}

#[derive(Debug, Parser)]
pub struct ChaCha20Opts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long, value_parser = verify_path)]
    pub output: String,
}

#[derive(Debug, Parser)]
pub struct ChaCha20GenerateOpts {
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}
