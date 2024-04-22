use clap::Parser;
use rcli::{process_csv, Cli, SubCommand};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
