use std::io::{self, Read};

use ayarify::ayarify;
use clap::Parser;

#[derive(Debug, Parser)]
struct Options {
    #[clap(short, long, default_value = "data-ayarify")]
    attribute: String,
}

fn main() {
    let Options { attribute } = Options::parse();
    let attr = if attribute.is_empty() { None } else { Some(attribute) };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("error while reading from STDIN");
    let output = ayarify(&input, attr.as_deref());
    println!("{}", output);
}
