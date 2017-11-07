#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

#![allow(let_and_return)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate config;

mod runner;
mod parser;
mod performance_calculator;
mod nicehash_api_raw;
mod nicehash_api;
mod benchmark;
mod nicehash_cpuminer_mapper;
mod algorithm_picker;
mod configuration_provider;

use std::env;

const DEV_WALLET: &str = "393EZrk5mwZ6gdVYmX5nguesVVJwxD9X2U";
const BENCHMARK_TIME_MS: u64 = 100_000;
const LOCATION: &str = "eu";

fn main() {
    // Work around openssl/trust/travis bug https://github.com/japaric/trust/issues/81 .
    env::set_var("SSL_CERT_DIR", "/etc/ssl/certs");
    let nicehash_response = nicehash_api::get_profitability().unwrap();
    let algorithms= nicehash_response.result.simplemultialgo;
    let benchmark = benchmark::benchmark(&algorithms, LOCATION, BENCHMARK_TIME_MS, &configuration_provider::get_configuration().cpuminer_multi_path, DEV_WALLET);

    let best_algorithm = algorithm_picker::pick_cpuminer_algorithm(&algorithms, &benchmark);

    println!("The most profitable algorithm to run at the moment is {:#?}", best_algorithm);
}
