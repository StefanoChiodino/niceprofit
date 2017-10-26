use clap::App;

const CPUMINER_MULTI_PATH_ARG: &str = "cpuminer-path";
const CPUMINER_MULTI_PATH_DEFAULT: &str = "cpuminer";

pub struct Configuration {
    pub cpuminer_multi_path: String,
}


pub fn get_configuration<'a>() -> &'a Configuration {
    lazy_static! {
        static ref CONFIGURATION: Configuration = init_configuration();
    }
    &CONFIGURATION
}

fn init_configuration() -> Configuration {
    let clap_yaml = load_yaml!("clap.yml");
    let matches = App::from_yaml(clap_yaml).get_matches();

    //    let user_wallet = matches.value_of(&USER_WALLET_ARG).unwrap_or(&DEV_WALLET);
    let cpuminer_multi_path = matches.value_of(&CPUMINER_MULTI_PATH_ARG).unwrap_or(CPUMINER_MULTI_PATH_DEFAULT);

    Configuration{cpuminer_multi_path : cpuminer_multi_path.to_string()}
}

#[test]
fn can_map_algorithm(){
    let configuration = get_configuration();

    assert_ne!(configuration.cpuminer_multi_path, "");
}
