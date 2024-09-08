use clap::Parser;
use rcli::{Cli, CmdExecutor, SubCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Support `RUST_LOG=debug cargo run`
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.cmd {
        SubCommand::Csv(opts) => opts.execute().await,
        SubCommand::GenPass(opts) => opts.execute().await,
        SubCommand::Base64(cmd) => cmd.execute().await,
        SubCommand::Text(cmd) => cmd.execute().await,
        SubCommand::Http(cmd) => cmd.execute().await,
        SubCommand::ChaCha20(cmd) => cmd.execute().await,
    }
}
