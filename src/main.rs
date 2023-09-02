use std::io;

use ayarify::ayarify;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long)]
    attribute: Option<Option<String>>,
}

fn main() {
    let options = Options::from_args();
    let attr = options.attribute.map(|a| a.unwrap_or("data-ayarify".into()));

    let stdin = io::stdin();
    ayarify(&mut stdin.lock(), &mut io::stdout(), attr.as_deref()).expect("io error");
}
