
use minifier::json::minify_from_read;
use structopt::StructOpt;
use std::{path::PathBuf, fs::{self, File}, io::{self, Read}};
use serde_json::Value;

#[derive(StructOpt, Debug)]
pub enum Json {
    /// Get a key from the document
    Key {
        key: String,

        #[structopt(short="f", parse(from_os_str))]
        input_file: PathBuf,
    },

    /// prettify JSON
    Expand {
        #[structopt(parse(from_os_str))]
        input_file: PathBuf,
    },

    /// compress the JSON to a single line minified version
    Minify {
        #[structopt(parse(from_os_str))]
        input_file: PathBuf,
    },
}

impl Json {
    pub fn run(&self) {
        match self {
            Json::Key { key, input_file } => {
                let json = read_json(input_file.into()).unwrap();
                println!("{}", &json[key]);
            },
            Json::Expand { input_file } => {
                let json = read_json(input_file.into()).unwrap();
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            },
            Json::Minify { input_file } => {
                let mut minified = String::new();
                let mut file = File::open(input_file).expect("file not found");
                minify_from_read(file).read_to_string(&mut minified);
                println!("{}", minified);
            },
        }
    }
}

fn read_json(input_file: PathBuf) -> io::Result<Value> {
    let json = fs::read_to_string(input_file)?;
    let parsed: Value = serde_json::from_str(&json)?;
    Ok(parsed)
}
