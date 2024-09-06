use std::fs;

use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    b64_decode, b64_encode, process_csv, process_generate_keys, process_genpass,
    process_http_serve, process_text_sign, process_text_verify, Base64SubCommand,
    ChaCha20SubCommand, Cli, HttpSubcommand, SignFormat, SubCommand, TextSubCommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Support `RUST_LOG=debug cargo run`
    tracing_subscriber::fmt::init();

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
        SubCommand::GenPass(genpass_opts) => {
            let passwd = process_genpass(
                genpass_opts.length,
                genpass_opts.noupper,
                genpass_opts.nolower,
                genpass_opts.nonumber,
                genpass_opts.nosymbol,
            )?;
            println!("{}", passwd);
            eprintln!("{}", zxcvbn(&passwd, &[]).score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = b64_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = b64_decode(&opts.input, opts.format)?;
                print!("{}", String::from_utf8_lossy(&decoded));
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", BASE64_URL_SAFE_NO_PAD.encode(sig));
            }
            TextSubCommand::Verify(opts) => {
                let res = process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("{}", res);
            }
            TextSubCommand::Generate(opts) => {
                let keys = process_generate_keys(opts.format)?;
                match opts.format {
                    SignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &keys[0])?;
                    }
                    SignFormat::Ed25519 => {
                        let name = opts.output.join("ed25519.sk");
                        fs::write(name, &keys[0])?;
                        let name = opts.output.join("ed25519.pk");
                        fs::write(name, &keys[1])?;
                    }
                }
            }
        },
        SubCommand::ChaCha20(subcmd) => match subcmd {
            ChaCha20SubCommand::Encrypt(opts) => {
                println!("{:?}", opts);
            }
            ChaCha20SubCommand::Decrypt(opts) => {
                println!("{:?}", opts)
            }
            ChaCha20SubCommand::Generate(opts) => {
                println!("{:?}", opts)
            }
        },
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubcommand::Serve(opts) => process_http_serve(opts.dir, opts.port).await?,
        },
    }
    Ok(())
}
