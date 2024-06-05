use std::{env, fs};
use renchmark_utils::include::parser::parse_config;
use renchmark_utils::parse_config2;

fn main() {
    let files: Vec<String> = env::args().skip(1).collect();
    let mut config = String::new();
    // 1. Parse config file
    for file in files.iter() {
        let file_content = fs::read_to_string(file).expect("File {file} not found");
        config.push_str(file_content.as_str());
    }
    parse_config(config);
    // 2. Install renchmark tracker on app nodes
    // 3. Install renchmark on any orchestrate_nodes
    //  -- wait 1 minute to establish basic usages
    // 4. RENCHMARK
    // 5. Gather information from renchmark tracker
    // 6. Delete renchmark tracker
    // 7. Generate reports
    //  -- Command line TUI
    //  -- JSON to file from config
    //  -- VUE HTML one file
}
