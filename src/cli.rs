mod base64;
mod chacha20;
mod csv;
mod genpass;
mod http;
mod text;

use crate::CmdExecutor;
pub use base64::*;
pub use chacha20::ChaCha20SubCommand;
use clap::Parser;
pub use csv::*;
pub use genpass::*;
pub use http::*;
use std::path::{Path, PathBuf};
pub use text::*;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about = None, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(name = "base64", about = "Base64 encode/decode")]
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(name = "text", about = "Text signature")]
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(name = "chacha20", about = "Encrypt/Decrypt with ChaCha20")]
    #[command(subcommand)]
    ChaCha20(ChaCha20SubCommand),
    #[command(name = "http", about = "HTTP server", subcommand)]
    Http(HttpSubcommand),
}

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await,
            SubCommand::GenPass(opts) => opts.execute().await,
            SubCommand::Base64(cmd) => cmd.execute().await,
            SubCommand::Text(cmd) => cmd.execute().await,
            SubCommand::Http(cmd) => cmd.execute().await,
            SubCommand::ChaCha20(cmd) => cmd.execute().await,
        }
    }
}

fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists".into())
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("You provided a invalid path!")
    }
}

#[cfg(test)]
mod tests {
    use super::verify_file;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("a.txt"), Err("File does not exists".into()))
    }
}
