use clap::Parser;
use rcli::{
    b64_decode, b64_encode, process_csv, process_genpass, Base64SubCommand, Cli, SubCommand,
};

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
        SubCommand::GenPass(genpass_opts) => process_genpass(
            genpass_opts.length,
            genpass_opts.noupper,
            genpass_opts.nolower,
            genpass_opts.nonumber,
            genpass_opts.nosymbol,
        )?,
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                b64_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                b64_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
