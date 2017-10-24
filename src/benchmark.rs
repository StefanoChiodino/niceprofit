use std::collections::HashMap;
use parser;
use performance_calculator;
use runner;

use std::process::Command;

pub fn benchmark(algorithms: Vec<&str>,
                 location: String,
                 benchmark_time_ms: u64,
                 cpuminer_multi_path: String,
                 wallet: String)
                 -> HashMap<&str, Option<f64>> {
    let benchmarks: HashMap<&str, Option<f64>> = algorithms
        .iter()
        .map(|a| {
            let mut cpuminer_multi_command = Command::new(&cpuminer_multi_path);
            cpuminer_multi_command
                .arg("-a cryptonight".to_string())
                .arg(format!("-o stratum+tcp://{}.{}.nicehash.com:3355", a, location))
                .arg(format!("-u {}.nicehash-cpumulti-miner-optimiser", wallet))
                .arg("-p x".to_string());


            let output = runner::run(cpuminer_multi_command, benchmark_time_ms);
            match output {
                Ok(o) => {
                    let parsed_output = parser::parse(o);
                    let hashrate = performance_calculator::calculate_hashrate(parsed_output);
                    (*a, Some(hashrate))
                }
                // TODO: don't hide errors.
                Err(_) => (*a, None),
            }
        })
        .collect();

    benchmarks
}