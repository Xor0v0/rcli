use super::{verify_file, verify_path};
use crate::{process_generate_keys, process_text_sign, process_text_verify, CmdExecutor};
use anyhow::{Ok, Result};
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use std::{fmt, path::PathBuf, str::FromStr};
use tokio::fs;

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

impl CmdExecutor for TextSubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::Generate(opts) => opts.execute().await,
        }
    }
}

impl CmdExecutor for SignOpts {
    async fn execute(self) -> Result<()> {
        let sig = process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", BASE64_URL_SAFE_NO_PAD.encode(sig));
        Ok(())
    }
}

impl CmdExecutor for VerifyOpts {
    async fn execute(self) -> Result<()> {
        let res = process_text_verify(&self.input, &self.key, &self.sig, self.format)?;
        println!("{}", res);
        Ok(())
    }
}

impl CmdExecutor for GenerateOpts {
    async fn execute(self) -> Result<()> {
        let keys = process_generate_keys(self.format)?;
        match self.format {
            SignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                fs::write(name, &keys[0]).await?;
            }
            SignFormat::Ed25519 => {
                let name = self.output.join("ed25519.sk");
                fs::write(name, &keys[0]).await?;
                let name = self.output.join("ed25519.pk");
                fs::write(name, &keys[1]).await?;
            }
        }
        Ok(())
    }
}
