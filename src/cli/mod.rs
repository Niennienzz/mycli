use std::path::{Path, PathBuf};

use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use self::chacha::*;
pub use self::http::*;
pub use self::jwt::*;

mod chacha;
mod jwt;
mod http;

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
    #[command(subcommand, about = "A simple HTTP file server")]
    Http(HttpSubCommand),
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

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

const MIN_KEY_LEN: usize = 8;

const VERIFY_KEY_ERR: &str = concat!("Key must be at least ", stringify!(MIN_KEY_LEN), " characters long");

fn verify_key(key: &str) -> Result<String, String> {
    if key.len() >= MIN_KEY_LEN {
        Ok(key.into())
    } else {
        Err(VERIFY_KEY_ERR.into())
    }
}

fn verify_jwt_audience(aud: &str) -> Result<String, &'static str> {
    if crate::utils::ALLOWED_JWT_AUDIENCES.contains(&aud) {
        Ok(aud.into())
    } else {
        Err("Invalid audience")
    }
}
