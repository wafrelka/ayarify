use std::io;

use ayarify::ayarify;
use clap::Parser;

#[derive(Debug, Parser)]
struct Options {
    #[clap(short, long)]
    attribute: Option<Option<String>>,
}

fn main() {
    let options = Options::parse();
    let attr = options.attribute.map(|a| a.unwrap_or("data-ayarify".into()));

    let stdin = io::stdin();
    ayarify(&mut stdin.lock(), &mut io::stdout(), attr.as_deref()).expect("io error");
}
