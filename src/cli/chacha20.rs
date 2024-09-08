use std::path::PathBuf;

use clap::Parser;

use crate::CmdExecutor;

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

impl CmdExecutor for ChaCha20SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            ChaCha20SubCommand::Encrypt(opts) => opts.execute().await,
            ChaCha20SubCommand::Decrypt(opts) => opts.execute().await,
            ChaCha20SubCommand::Generate(opts) => opts.execute().await,
        }
    }
}

impl CmdExecutor for ChaCha20Opts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("ChaCha20Opts: {:?}", self);
        Ok(())
    }
}

impl CmdExecutor for ChaCha20GenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("ChaCha20GenerateOpts: {:?}", self);
        Ok(())
    }
}
