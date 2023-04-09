use std::{fs, io, path::PathBuf, str::FromStr};

use crate::error::SakError;
use base64::{engine::general_purpose, Engine as _};
use hex;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Encode {
    /// Output encoding type.
    /// One of hex, base64, ascii
    #[structopt(short, long)]
    to: EncodingOption,

    /// the file to encode/decode, use `-` for stdin
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
}

impl Encode {
    pub fn run(&self) {
        match self.process() {
            Ok(string) => println!("{}", string),
            Err(e) => println!("error: {}", e),
        }
    }

    fn process(&self) -> Result<String, SakError> {
        let input_data = if self.input_file.display().to_string() == "-" {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            buffer
        } else {
            fs::read_to_string(&self.input_file)?
        };

        match self.to {
            EncodingOption::Hex => Ok(self.convert_hex(input_data)),
            EncodingOption::Base64 => Ok(self.convert_base64(input_data)),
            EncodingOption::ASCII => self.convert_ascii(input_data),
        }
    }

    fn convert_hex(&self, input_data: String) -> String {
        hex::encode(input_data)
    }

    fn convert_base64(&self, input_data: String) -> String {
        general_purpose::STANDARD.encode(&input_data.as_bytes())
    }

    fn convert_ascii(&self, input_data: String) -> Result<String, SakError> {
        let encoded = std::str::from_utf8(&input_data.as_bytes())?;
        Ok(encoded.to_owned())
    }
}

/// Encoding formats
#[derive(StructOpt, Debug)]
pub enum EncodingOption {
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
            _ => Err(SakError::InvalidArgument(format!(
                "invalid encoding option `{}`",
                s
            ))),
        }
    }
}
