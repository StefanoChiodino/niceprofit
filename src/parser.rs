extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;

pub struct Interpretation {
    cpu_count: u32,
}

pub fn parse(output: String) -> Interpretation {

    lazy_static! {
        static ref regex: Regex = Regex::new(r"CPU #(\d): (\d.?\d+) (.*H)/s").unwrap();
    }

    let mut cpu_stats: HashMap<String, Vec<String>> = HashMap::new();

    /* regex.captures_iter(&output)
        .into_iter()
        .group_by(|line| line[0]) */



    for capture in regex.captures_iter(&output) {
        let core_number = capture[1].to_string();
        let hashrate = capture[2].to_string();
        if cpu_stats.contains_key(&core_number) {
            cpu_stats.get_mut(&core_number).unwrap().push(hashrate);
        } else {
            cpu_stats.insert(core_number, vec![hashrate]);
        }
    }

    let interpretation = Interpretation { cpu_count: cpu_stats.len() as u32 };

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

    assert_eq!(interpretation.cpu_count, 4);
}