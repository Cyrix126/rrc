use std::{f64, path::PathBuf, process::exit};

use clap::Parser;
use get_pass::get_password;
use reqwest::blocking::ClientBuilder;
use rrc_lib::client::RakutenClientBuilder;
use serde::{Deserialize, Serialize};

fn main() {
    let args = Cli::parse();
    let cfg = confy::load::<Config>("rrc-price-update", "config")
        .expect("Can not load the configuration file");
    let rrc = RakutenClientBuilder::default()
        .client(ClientBuilder::new())
        .unwrap()
        .token(get_password(&cfg.token).unwrap())
        .username(cfg.username)
        .update_price_pf_nb(cfg.update_price_pf_nb)
        .build()
        .unwrap();

    if let Some(skus) = args.skus {
        rrc.update_price(skus.into_iter().map(|sku| (sku, args.new_price)).collect())
            .unwrap();
        exit(0);
    }

    let mut products = rrc.export().unwrap();
    products.retain(|p| p.sku.is_some());
    let skus_price: Vec<(String, f32)> = products
        .into_iter()
        .filter(|p| {
            p.price.amount >= args.take_sku_with_price_min as f64
                && p.price.amount <= args.take_sku_with_price_max as f64
        })
        .map(|p| (p.sku.unwrap(), args.new_price))
        .collect();
    rrc.update_price(skus_price).unwrap();
}

#[derive(clap::Parser)]
struct Cli {
    #[arg(short = '>', long)]
    take_sku_with_price_min: f32,
    #[arg(short = '<', long)]
    take_sku_with_price_max: f32,
    #[arg(short = '=', long)]
    new_price: f32,
    pub skus: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Default)]
struct Config {
    token: PathBuf,
    username: String,
    update_price_pf_nb: u32,
}
