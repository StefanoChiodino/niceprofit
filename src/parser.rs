extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;

pub struct Interpretation {
    cpu_hashrates: HashMap<u32, Vec<f64>>,
}

pub fn parse(output: String) -> Interpretation {

    lazy_static! {
        static ref regex: Regex = Regex::new(r"CPU #(\d): (\d.?\d+) (.*H)/s").unwrap();
    }

    let mut cpu_hashrates: HashMap<u32, Vec<f64>> = HashMap::new();

    for capture in regex.captures_iter(&output) {
        let core_number = capture[1].parse().unwrap();
        let hashrate = capture[2].parse().unwrap();
        if cpu_hashrates.contains_key(&core_number) {
            cpu_hashrates
                .get_mut(&core_number)
                .unwrap()
                .push(hashrate);
        } else {
            cpu_hashrates.insert(core_number, vec![hashrate]);
        }
    }

    let interpretation = Interpretation { cpu_hashrates: cpu_hashrates };

    interpretation
}

#[test]
fn test_parse_cpumulti_miner_output() {
    let output = "CPU #2: 1.95 H/s
CPU #1: 2.21 H/s
CPU #3: 2.23 H/s
stratum_recv_line failed
Stratum connection interrupted
CPU #0: 2.07 H/s
CPU #2: 1.91 H/s
CPU #2: 1.50 H/s
CPU #1: 1.80 H/s"
            .to_string();

    let interpretation = parse(output);

    assert_eq!(interpretation.cpu_hashrates.len(), 4);
    assert_eq!(interpretation.cpu_hashrates[&(0 as u32)].len(), 1);
    assert_eq!(interpretation.cpu_hashrates[&(1 as u32)].len(), 2);
}
