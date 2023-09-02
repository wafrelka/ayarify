use std::io;

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

    let stdin = io::stdin();
    ayarify(&mut stdin.lock(), &mut io::stdout(), attr.as_deref()).expect("io error");
}
