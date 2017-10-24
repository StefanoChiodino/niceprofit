use std::collections::HashMap;
use parser;
use performance_calculator;
use runner;
use profitability::Simplealgo;

use std::process::Command;

pub fn benchmark(algorithms: Vec<Simplealgo>,
                 location: &str,
                 benchmark_time_ms: u64,
                 cpuminer_multi_path: &str,
                 wallet: &str)
                 -> HashMap<String, Result<f64, String>> {
    let benchmarks: HashMap<String, Result<f64, String>> = algorithms
        .iter()
        .map(|simplemultialgo| {
            let mut cpuminer_multi_command = Command::new(&cpuminer_multi_path);
            cpuminer_multi_command
                .arg("-a".to_string())
                .arg(simplemultialgo.name.to_string())
                .arg("-o")
                .arg(format!("stratum+tcp://{}.{}.nicehash.com:{}", simplemultialgo.name, location, simplemultialgo.port))
                .arg("-u")
                .arg(format!("{}.nicehash-cpumulti-miner-optimiser", wallet))
                .arg("-p")
                .arg("x");

            println!("Running '{:?}'", cpuminer_multi_command);

            let output = runner::run(cpuminer_multi_command, benchmark_time_ms);
            match output {
                Ok(Some(o)) => {
                    let parsed_output = parser::parse(o);
                    let hashrate = performance_calculator::calculate_hashrate(parsed_output);
                    (simplemultialgo.name.to_string(), Ok(hashrate))
                },
                Ok(None) => (simplemultialgo.name.to_string(), Err("no output!".to_string())),
                Err(e) => (simplemultialgo.name.to_string(), Err(e.to_string())),
            }
        })
        .collect();

    benchmarks
}

#[test]
fn can_benchmark_algorithms(){
    let algorithm_name = "cryptonight";
    let algorithm = Simplealgo{
        name: algorithm_name.to_string(),
        port:3355_u32,
        algo:0_u32,
        paying: 0_f64,
    };
    let mut benchmarks = benchmark(
        vec![algorithm],
        &::LOCATION,
        ::BENCHMARK_TIME_MS,
        &::CPUMINER_MULTI_PATH,
        &::DEV_WALLET);
    let mut algorithm_benchmark = benchmarks.get_mut(algorithm_name).unwrap();

    match *algorithm_benchmark {
        Err(ref e) => panic!(e.clone()),
        _=>()
    }
}
