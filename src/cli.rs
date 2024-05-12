use std::path::Path;

use clap::Parser;

mod base64;
mod csv;
mod genpass;

pub use self::base64::{Base64Format, Base64SubCommand};
pub use self::csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;

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
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exists".into())
    }
}

#[cfg(test)]
mod tests {
    use super::verify_input_file;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("a.txt"),
            Err("File does not exists".into())
        )
    }
}
