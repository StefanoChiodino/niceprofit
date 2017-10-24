use std::collections::HashMap;

pub fn calculate_hashrate(interpretation: HashMap<u32, Vec<f64>>) -> f64 {
    let hashrate: f64 = interpretation
        .iter()
        .map(|(_, v)| v.iter().sum::<f64>() / v.len() as f64)
        .sum::<f64>();

    hashrate
}

#[test]
fn test_hashrate_calcualation() {
    let interpretation: HashMap<u32, Vec<f64>> = [(0, vec![0_f64, 2_f64]), (1, vec![3_f64, 3_f64])]
        .iter()
        .cloned()
        .collect();

    let hashrate = calculate_hashrate(interpretation);

    assert_eq!(hashrate, 4 as f64);
}
