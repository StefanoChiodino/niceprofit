use std::collections::HashMap;
use parser;
use performance_calculator;
use runner;
use profitability::ProfitabilityError;

use std::process::Command;

pub fn benchmark(algorithms: Vec<String>,
                 location: &str,
                 benchmark_time_ms: u64,
                 cpuminer_multi_path: &str,
                 wallet: &str)
                 -> HashMap<String, Result<f64, String>> {
    let benchmarks: HashMap<String, Result<f64, String>> = algorithms
        .iter()
        .map(|a| {
            let mut cpuminer_multi_command = Command::new(&cpuminer_multi_path);
            cpuminer_multi_command
                .arg("-a")
                .arg(a)
                .arg("-o")
                .arg(format!("stratum+tcp://{}.{}.nicehash.com:3355", a, location))
                .arg("-u")
                .arg(format!("{}.nicehash-cpumulti-miner-optimiser", wallet))
                .arg("-p")
                .arg("x");

            let output = runner::run(cpuminer_multi_command, benchmark_time_ms);
            match output {
                Ok(Some(o)) => {
                    let parsed_output = parser::parse(o);
                    let hashrate = performance_calculator::calculate_hashrate(parsed_output);
                    (a.clone(), Ok(hashrate))
                },
                Ok(None) => (a.clone(), Err("no output!".to_string())),
                Err(e) => (a.clone(), Err(e.to_string())),
            }
        })
        .collect();

    benchmarks
}

#[test]
fn can_benchmark_algorithms(){
    let algorithm = "cryptonight".to_string();
    let benchmarks = benchmark(
        vec![algorithm.clone()],
        &::LOCATION,
        ::BENCHMARK_TIME_MS,
        &::CPUMINER_MULTI_PATH,
        &::DEV_WALLET);
    let algorithm_benchmark = benchmarks[&algorithm].clone();

    match algorithm_benchmark {
        Err(ref e) => panic!(e.clone()),
        _=>()
    }
}
