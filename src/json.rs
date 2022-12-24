
use minifier::json::minify_from_read;
use structopt::StructOpt;
use std::{path::PathBuf, fs::File, io::Read};

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
        #[structopt(short, long, default_value="4")]
        indent_size: i32,
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
            Json::Key { key, input_file } => todo!(),
            Json::Expand { indent_size, input_file } => todo!(),
            Json::Minify { input_file } => {
                let mut minified = String::new();
                let mut file = File::open(input_file).expect("file not found");
                minify_from_read(file).read_to_string(&mut minified);
                println!("{}", minified);
            },
        }
    }
}
