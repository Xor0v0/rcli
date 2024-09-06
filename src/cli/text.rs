use std::{fmt, path::PathBuf, str::FromStr};

use super::{verify_file, verify_path};
use anyhow::Ok;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign a message with a key")]
    Sign(SignOpts),
    #[command(name = "verify", about = "Verify a signed message")]
    Verify(VerifyOpts),
    #[command(name = "generate", about = "Generate keys")]
    Generate(GenerateOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: SignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: SignFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser = parse_format)]
    pub format: SignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum SignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for SignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(SignFormat::Blake3),
            "ed25519" => Ok(SignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

// &str 的 parse() ，需要要 impl FromStr for T
fn parse_format(format: &str) -> Result<SignFormat, anyhow::Error> {
    format.parse()
}

impl From<SignFormat> for &'static str {
    fn from(format: SignFormat) -> Self {
        match format {
            SignFormat::Blake3 => "blake3",
            SignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for SignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", Into::<&str>::into(self))
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
