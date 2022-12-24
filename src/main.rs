#![allow(unused)]
mod json;

use crate::json::Json;
use std::{fmt::Display, path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Swiss army knife")]
enum Sak {
    /// Json utility functions
    Json(Json),
    /// Json Web Token utilities
    Jwt(Jwt),
    /// hashing functions
    Hashing(Hashing),
    /// Encode/decode values from a variety of formats
    Encode(Encode),
}

#[derive(StructOpt, Debug)]
struct Jwt {}

#[derive(StructOpt, Debug)]
struct Hashing {
    /// text to hash
    #[structopt(long, short)]
    data: Option<String>,

    /// The JSON file to operate on, use `-` for stdin
    #[structopt(parse(from_os_str))]
    input_file: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
struct Encode {
    /// Output encoding type.
    /// One of hex, base64, ascii
    #[structopt(short, long)]
    to: EncodingOption,

    /// The encoding the input data is in.
    /// One of hex, base64, ascii
    #[structopt(short, long)]
    from: EncodingOption,

    /// the file to encode/decode, use `-` for stdin
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
}

/// Encoding formats
#[derive(StructOpt, Debug)]
enum EncodingOption {
    Hex,
    Base64,
    ASCII,
}

impl FromStr for EncodingOption {
    type Err = SakError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hex" => Ok(EncodingOption::Hex),
            "base64" => Ok(EncodingOption::Base64),
            "ascii" => Ok(EncodingOption::ASCII),
            _ => Err(SakError::ParseError(format!(
                "invalid encoding option `{}`",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone)]
enum SakError {
    ParseError(String),
}

impl Display for SakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::SakError::*;

        match self {
            ParseError(value) => write!(f, "could not parse value `{}`", value),
        }
    }
}

fn main() {
    let sak = Sak::from_args();

    match sak {
        Sak::Json(subcommand) => subcommand.run(),
        Sak::Jwt(subcommand) => todo!(),
        Sak::Hashing(subcommand) => todo!(),
        Sak::Encode(subcommand) => todo!(),
    }
}
