#![allow(unused)]
mod encode;
mod error;
mod json;

use crate::json::Json;
use std::{fmt::Display, path::PathBuf, str::FromStr};

use encode::Encode;
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

fn main() {
    let sak = Sak::from_args();

    match sak {
        Sak::Json(subcommand) => subcommand.run(),
        Sak::Jwt(subcommand) => todo!(),
        Sak::Hashing(subcommand) => todo!(),
        Sak::Encode(subcommand) => subcommand.run(),
    }
}
