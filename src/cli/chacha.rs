use std::path::Path;

use clap::Parser;

use crate::CmdExecutor;
use crate::process::chacha::ChaCha;

#[derive(Debug, Parser)]
pub enum ChaChaSubCommand {
    #[command(name = "encrypt", about = "Encrypt a file with ChaCha20-Poly1305")]
    Encrypt(ChaChaOpts),
    #[command(name = "decrypt", about = "Decrypt a file with ChaCha20-Poly1305")]
    Decrypt(ChaChaOpts),
}

impl CmdExecutor for ChaChaSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        let result = match self {
            ChaChaSubCommand::Encrypt(opts) => {
                let mut reader = crate::get_reader(&opts.input)?;
                ChaCha::process_encrypt(&opts.key, &mut reader)?
            }
            ChaChaSubCommand::Decrypt(opts) => {
                let mut reader = crate::get_reader(&opts.input)?;
                ChaCha::process_decrypt(&opts.key, &mut reader)?
            }
        };
        println!("{}", result);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct ChaChaOpts {
    #[arg(long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long)]
    pub key: String,
}

// Helper function to verify file existence.
// It also allows "-" as a special case for console input.
fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
