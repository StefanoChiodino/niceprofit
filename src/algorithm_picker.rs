use nicehash_api::Simplealgo;
use std::collections::HashMap;
use std::iter::Iterator;
use std::cmp::Ordering;

pub fn pick_cpuminer_algorithm(simplealgos: &[Simplealgo],
                               benchmarks: &HashMap<String, Result<f64, String>>)
                               -> String {
    let mut zip = simplealgos
        .iter()
        .map(|a| {
            let benchmark = benchmarks.get(&a.name);
            let profitability = get_profitability(a, benchmark.unwrap_or(&Err("no benchmark available for this algorithm".to_string())));
            println!("Algorithm {:?} is paying {:?}, benchmark '{:?}' H/s, profitability '{:?}'", a.name, a.paying, benchmark, profitability);
            (a, benchmark, profitability)
        })
        .collect::<Vec<_>>();
    zip
        .sort_by(|&(_, _, ref p1), &(_, _, ref p2)| {
            let ordering = p1
                .partial_cmp(p2)
                .unwrap_or(Ordering::Less);
            ordering
        });

    let most_profitable = zip.iter().last();

    most_profitable.unwrap().0.name.to_string()
}

fn get_profitability(simplealgo: &Simplealgo, benchmark: &Result<f64, String>) -> f64 {
    let profitability = simplealgo.paying * benchmark.clone().unwrap_or(0_f64);
    profitability
}

#[test]
fn can_get_most_profitable() {
    let simplealgos = vec![Simplealgo {
        paying: 10f64,
        name: "a".to_string(),
        algo: 0,
        port: 0,
    },
                           Simplealgo {
                               paying: 0f64,
                               name: "b".to_string(),
                               algo: 0,
                               port: 0,
                           },
                           Simplealgo {
                               paying: 1f64,
                               name: "c".to_string(),
                               algo: 0,
                               port: 0,
                           },
                           Simplealgo {
                               paying: 1f64,
                               name: "d".to_string(),
                               algo: 0,
                               port: 0,
                           }];
    let benchmarks = [("a".to_string(), Err("error".to_string())),
        ("b".to_string(), Ok(1f64)),
        ("c".to_string(), Ok(1f64)),
        ("d".to_string(), Ok(2f64))]
        .iter()
        .cloned()
        .collect();

    let algorithm = pick_cpuminer_algorithm(&simplealgos, &benchmarks);

    assert_eq!(algorithm, "d")
}
