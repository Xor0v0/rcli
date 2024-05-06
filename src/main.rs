use clap::Parser;
use rcli::{process_csv, Cli, SubCommand};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?;
        }
    }
    Ok(())
}
