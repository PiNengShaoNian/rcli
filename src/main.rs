use std::fs;

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify, Base64SubCommand,
    HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(opts.input.as_str(), output, opts.format)?
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);

            // output password strength in stderr
            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(opts.input.as_str(), opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(opts.input.as_str(), opts.format)?;
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let signed = process_text_sign(opts.input.as_str(), &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified =
                    process_text_verify(opts.input.as_str(), &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
                println!("{:?}", key);
            }
        },
        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => process_http_serve(opts.path, opts.port).await?,
        },
    }
    anyhow::Ok(())
}
