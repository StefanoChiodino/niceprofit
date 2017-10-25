extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;

pub fn parse(output: String) -> HashMap<u32, Vec<f64>> {

    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"CPU #(\d): (.+) (.*H)/s").unwrap();
    }

    let mut cpu_hashrates: HashMap<u32, Vec<f64>> = HashMap::new();
    for capture in REGEX.captures_iter(&output) {
        let core_number = capture[1].parse().unwrap();
        let mut hashrate = capture[2].parse().unwrap();
        print!("{:?}", capture);
        match &capture[3] {
            "kH" => hashrate *= 1_000_f64,
            "mH" => hashrate *= 1_000_000_f64,
            "gH" => hashrate *= 1_000_000_000_f64,
            "tH" => hashrate *= 1_000_000_000_000_f64,
            "pH" => hashrate *= 1_000_000_000_000_000_f64,
            _ => (),
        }
        if cpu_hashrates.contains_key(&core_number) {
            cpu_hashrates
                .get_mut(&core_number)
                .unwrap()
                .push(hashrate);
        } else {
            cpu_hashrates.insert(core_number, vec![hashrate]);
        }
    }

    cpu_hashrates
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

    assert_eq!(interpretation.len(), 4);
    assert_eq!(interpretation[&(0 as u32)].len(), 1);
    assert_eq!(interpretation[&(1 as u32)].len(), 2);
    assert_eq!(interpretation[&(0 as u32)][0], 2.07);
}

#[test]
fn test_parsing_khash() {
    let output = "CPU #0: 0.25 kH/s"
            .to_string();

    let interpretation = parse(output);

    assert_eq!(interpretation[&(0 as u32)][0], 250_f64);
}


#[test]
fn test_calculate_hashrate() {
    let output = "CPU #0: 1 H/s
CPU #1: 1 kH/s
CPU #2: 1 mH/s
CPU #3: 1 gH/s
CPU #4: 1 tH/s
CPU #5: 1 pH/s"
            .to_string();

    let interpretation = parse(output);

    assert_eq!(interpretation[&(0 as u32)][0], 1_f64);
    assert_eq!(interpretation[&(1 as u32)][0], 1_000_f64);
    assert_eq!(interpretation[&(2 as u32)][0], 1_000_000_f64);
    assert_eq!(interpretation[&(3 as u32)][0], 1_000_000_000_f64);
    assert_eq!(interpretation[&(4 as u32)][0], 1_000_000_000_000_f64);
    assert_eq!(interpretation[&(5 as u32)][0], 1_000_000_000_000_000_f64);
}
