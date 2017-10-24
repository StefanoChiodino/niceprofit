extern crate serde_json;
extern crate serde;

extern crate reqwest;

use std::error::Error;
use std::fmt;
use std::str::FromStr;
use self::serde::de::{self, Deserialize, Deserializer};
use std::fmt::Display;

#[derive(Debug)]
pub enum ProfitabilityError {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    NotFound,
}

#[derive(Serialize, Deserialize)]
pub struct SimplemultialgoInfo {
    pub result: NicehashResult,
}

#[derive(Serialize, Deserialize)]
pub struct NicehashResult {
    pub simplemultialgo: Vec<Simplealgo>,
    method: String,
}

#[derive(Serialize, Deserialize)]
pub struct Simplealgo {
    #[serde(deserialize_with = "from_str")]
    paying: f64,
    port: u32,
    pub name: String,
    algo: u32,
}

pub fn get_profitability() -> Result<SimplemultialgoInfo, ProfitabilityError> {
    use query_nicehash;

    let profitability_json = query_nicehash::get_nicehash_response()?;

    let simplemultialgoinfo: SimplemultialgoInfo = serde_json::from_str(&profitability_json)?;

    Ok(simplemultialgoinfo)
}

impl fmt::Display for ProfitabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProfitabilityError::Reqwest(ref err) => err.fmt(f),
            ProfitabilityError::SerdeJson(ref err) => err.fmt(f),
            ProfitabilityError::NotFound => panic!("Error type not found"),
        }
    }
}

impl From<reqwest::Error> for ProfitabilityError {
    fn from(err: reqwest::Error) -> ProfitabilityError {
        ProfitabilityError::Reqwest(err)
    }
}

impl From<serde_json::Error> for ProfitabilityError {
    fn from(err: serde_json::Error) -> ProfitabilityError {
        ProfitabilityError::SerdeJson(err)
    }
}

impl Error for ProfitabilityError {
    fn description(&self) -> &str {
        match *self {
            ProfitabilityError::Reqwest(ref err) => err.description(),
            ProfitabilityError::SerdeJson(ref err) => err.description(),
            ProfitabilityError::NotFound => "not found",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ProfitabilityError::Reqwest(ref err) => Some(err),
            ProfitabilityError::SerdeJson(ref err) => Some(err),
            ProfitabilityError::NotFound => None,
        }
    }
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[test]
fn can_deserialise_nicehash_response() {
    let result = get_profitability();

    match result {
        Ok(r) => {
            assert!(true);
            assert_eq!(r.result.method, "simplemultialgo.info");
        }
        Err(e) => assert_ne!(format!("{:?}", e), ""),
    }
}
