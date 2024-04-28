use clap::Parser;

use crate::CmdExecutor;
use crate::process::jwt;

#[derive(Debug, Parser)]
pub enum JwtSubCommand {
    #[command(name = "sign", about = "Sign a JWT token")]
    Sign(JwtSignOpts),
    #[command(name = "verify", about = "Verify a JWT token")]
    Verify(JwtVerifyOpts),
}

impl CmdExecutor for JwtSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = match self {
            JwtSubCommand::Sign(opts) => {
                jwt::Jwt::process_sign(
                    &opts.key,
                    &jwt::JwtClaims {
                        sub: opts.sub,
                        aud: opts.aud,
                        exp: opts.exp,
                    },
                )?
            }
            JwtSubCommand::Verify(opts) => {
                jwt::Jwt::process_verify(&opts.key, &opts.token)?
            }
        };
        println!("{}", ret);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(long)]
    pub key: String,
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long)]
    pub exp: i64,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(long)]
    pub key: String,
    #[arg(long)]
    pub token: String,
}
