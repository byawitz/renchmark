use serde_yaml;
use crate::include::yaml::RenchmarkConfig;

pub fn parse_config(config: String) {
    let docs: RenchmarkConfig = serde_yaml::from_str(config.as_str()).unwrap();
    print!("a");
}