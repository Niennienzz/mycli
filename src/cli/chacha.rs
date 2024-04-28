use clap::Parser;

use crate::CmdExecutor;

use super::verify_file;

#[derive(Debug, Parser)]
pub enum ChaChaSubCommand {
    #[command(name = "encrypt", about = "Encrypt a file with ChaCha20-Poly1305")]
    Encrypt(ChaChaOpts),
    #[command(name = "decrypt", about = "Decrypt a file with ChaCha20-Poly1305")]
    Decrypt(ChaChaOpts),
}

#[derive(Debug, Parser)]
pub struct ChaChaOpts {
    #[arg(long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long)]
    pub key: String,
}

impl CmdExecutor for ChaChaSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = match self {
            ChaChaSubCommand::Encrypt(opts) => {
                let mut reader = crate::get_reader(&opts.input)?;
                crate::process_encrypt(&opts.key, &mut reader)?
            }
            ChaChaSubCommand::Decrypt(opts) => {
                let mut reader = crate::get_reader(&opts.input)?;
                crate::process_decrypt(&opts.key, &mut reader)?
            }
        };
        println!("{}", ret);
        Ok(())
    }
}
