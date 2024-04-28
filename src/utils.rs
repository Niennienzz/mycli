use std::{fs::File, io::Read};

use anyhow::Result;
use lazy_static::lazy_static;
use tera::Tera;

pub const ALLOWED_JWT_AUDIENCES: [&str; 3] = [
    "https://u.geekbang.org",
    "https://time.geekbang.org",
    "https://www.dragonflydb.io",
];

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

// Use 'lazy_static' to compile the Tera template only once.
lazy_static! {
    pub static ref DIRECTORY_LISTING: Tera = {
        let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}
