use nicehash_api::Simplealgo;
use std::collections::HashMap;
use std::iter::Iterator;
use std::cmp::Ord;
use std::cmp::Ordering;

struct Pair<A, B> {
    a: A,
    b: B,
}

pub fn pick_cpuminer_algorithm(simplealgos: Vec<Simplealgo>, benchmarks: HashMap<String, Result<f64, String>>) -> String {
    let mut zip = simplealgos
        .iter()
        .zip(benchmarks)
        .collect::<Vec<_>>();
        zip.sort_by(|&(ref a1, ref a2), &(ref b1, ref b2)| {
            let profitability_a = get_profitability(a1, a2.1.clone());
            let profitability_b = get_profitability(b1, b2.1.clone());
            profitability_b.partial_cmp(&profitability_a).unwrap_or(Ordering::Greater)
        });

        let most_profitable = zip.first();

    most_profitable.unwrap().0.name.to_string()
}

fn get_profitability(simplealgo: &Simplealgo, benchmark: Result<f64, String>) -> f64 {
    let profitability = simplealgo.paying * benchmark.unwrap_or(0_f64);
    profitability
}

#[test]
fn can_get_most_profitable() {
    let simplealgos = vec![
        Simplealgo { paying: 0f64, name: "a".to_string(), algo: 0, port: 0 },
        Simplealgo { paying: 0f64, name: "b".to_string(), algo: 0, port: 0 },
        Simplealgo { paying: 1f64, name: "c".to_string(), algo: 0, port: 0 },
        Simplealgo { paying: 1f64, name: "d".to_string(), algo: 0, port: 0 }];
    let benchmarks = [
        ("a".to_string(), Err("error".to_string())),
        ("b".to_string(), Ok(1f64)),
        ("c".to_string(), Ok(1f64)),
        ("d".to_string(), Ok(2f64)),
    ]
        .iter()
        .cloned()
        .collect();

    let algorithm = pick_cpuminer_algorithm(simplealgos, benchmarks);

    assert_eq!(algorithm, "d")
}
