use std::io::{self, Read};

use ayarify::{ayarify, AyarifyOptions};
use clap::Parser;

#[derive(Debug, Parser)]
struct Options {
    #[clap(short, long, default_value = "data-ayarify")]
    attribute: String,
    #[clap(short, long, default_value = "div")]
    element: String,
}

fn main() {
    let Options { attribute, element } = Options::parse();

    let options = AyarifyOptions {
        attribute: if attribute.is_empty() { None } else { Some(attribute) },
        element,
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("error while reading from STDIN");
    let output = ayarify(&input, options);
    println!("{}", output);
}
