#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use std::process::Command;

mod runner;
mod parser;
mod performance_calculator;
mod query_nicehash;
mod profitability;

const USER_WALLET_ARG: &str = "wallet";
const DEV_WALLET: &str = "393EZrk5mwZ6gdVYmX5nguesVVJwxD9X2U";
const CPUMINER_MULTI_PATH: &str = "cpumulti-miner";
const BENCHMARK_TIME_MS: u64 = 10000;
const LOCATION: &str = "eu";

fn main() {
    let yaml = load_yaml!("clap.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let user_wallet = matches.value_of(&USER_WALLET_ARG).unwrap_or(&DEV_WALLET);

    let mut cpuminer_multi_command = Command::new(CPUMINER_MULTI_PATH);
    cpuminer_multi_command
        .arg("-a cryptonight".to_string())
        .arg("-o stratum+tcp://cryptonight.eu.nicehash.com:3355".to_string())
        .arg(format!("-u {}.nicehash-cpumulti-miner-optimiser", user_wallet))
        .arg("-p x".to_string());

    let output = runner::run(cpuminer_multi_command, BENCHMARK_TIME_MS);
}
