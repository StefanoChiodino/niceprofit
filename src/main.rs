#[macro_use]
extern crate clap;
use clap::App;

use std::process::Command;


mod runner;

const user_wallet_arg: &str = "wallet";
const dev_wallet: &str = "393EZrk5mwZ6gdVYmX5nguesVVJwxD9X2U";
const cpuminer_multi_path: &str = "cpumulti-miner";
const benchmark_time_ms: u64 = 10000;

fn main() {
    let yaml = load_yaml!("clap.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let user_wallet = matches.value_of(&user_wallet_arg).unwrap_or(&dev_wallet);

    let mut cpuminer_multi_command = Command::new(cpuminer_multi_path);
    cpuminer_multi_command
        .arg("-a cryptonight".to_string())
        .arg("-o stratum+tcp://cryptonight.eu.nicehash.com:3355".to_string())
        .arg(format!("-u {}.nicehash-cpumulti-miner-optimiser", user_wallet))
        .arg("-p x".to_string());

    let output = runner::run(cpuminer_multi_command, benchmark_time_ms);

}
