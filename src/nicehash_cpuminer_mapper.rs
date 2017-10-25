extern crate config;
use config::*;
use std::collections::HashMap;

lazy_static! {
	static ref MAP: HashMap<String, Option<String>> = get_map();
}

pub fn get_cpuminer_algorithm_name<'a>(nicehash_algorithm_name: &String) -> Option<&'a Option<String>>{
    MAP.get(nicehash_algorithm_name)
}

fn get_map()-> HashMap<String, Option<String>> {
    let mut config = Config::default();
    config
        .merge(File::with_name("nicehash_cpuminer_map.json"))
        .unwrap();
    let map = config.deserialize::<HashMap<String, Option<String>>>().unwrap();

    map
}

#[test]
fn can_map_algorithm(){
    let algorithm = "cryptonight";

    assert_eq!(MAP.get(algorithm), Some(&Some(algorithm.to_string())));
}
