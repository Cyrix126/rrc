use std::path::PathBuf;

use clap::Parser;
use get_pass::get_password;
use reqwest::blocking::ClientBuilder;
use rrc_lib::{client::RakutenClientBuilder, export::HEADERS_CSV};
use serde::{Deserialize, Serialize};

fn main() {
    let args = Cli::parse();
    let cfg = confy::load::<Config>("rrc", "config").expect("Can not load the configuration file");
    let rrc = RakutenClientBuilder::default()
        .client(ClientBuilder::new())
        .unwrap()
        .token(get_password(&cfg.token).unwrap())
        .username(cfg.username)
        .build()
        .unwrap();
    let products = rrc.export().unwrap();
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .terminator(csv::Terminator::CRLF)
        .from_path(args.output)
        .unwrap();
    wtr.write_record(HEADERS_CSV).unwrap();
    for p in products {
        wtr.serialize(p).unwrap();
    }
}

#[derive(clap::Parser)]
struct Cli {
    #[arg(short, long)]
    output: PathBuf,
}
#[derive(Serialize, Deserialize, Default)]
struct Config {
    token: PathBuf,
    username: String,
}
