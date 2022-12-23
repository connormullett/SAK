#![allow(unused)]

use std::{fmt::Display, path::PathBuf, str::FromStr};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Swiss army knife")]
enum Sak {
    /// Json utility functions
    Json(Json),
    /// Json Web Token utilities
    Jwt,
    /// hashing functions
    Hashing,
    /// Encode/decode values from a variety of formats
    Encode(Encode),
}

#[derive(StructOpt, Debug)]
struct Json {
    /// Get a key from the document
    #[structopt(long, short)]
    key: Option<String>,

    /// prettify JSON
    #[structopt(long, short)]
    expand: bool,

    /// compress the JSON to a single line minified version
    #[structopt(long, short)]
    minify: bool, // minify the input file

    /// The JSON file to operate on, use `-` for stdin
    #[structopt(parse(from_os_str))]
    input_file: Vec<PathBuf>, // files to operate on
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
    println!("{:?}", sak);
}
