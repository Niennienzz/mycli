use std::path::{Path, PathBuf};

use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use self::chacha::*;
pub use self::jwt::*;

mod chacha;
mod jwt;

#[derive(Debug, Parser)]
#[command(name = "my-cli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(subcommand, name = "text", about = "Encrypt/decrypt text with ChaCha20-Poly1305")]
    ChaCha(ChaChaSubCommand),
    #[command(subcommand, name = "jwt", about = "Sign/verify JWT tokens")]
    Jwt(JwtSubCommand),
}

#[allow(dead_code)]
fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}
