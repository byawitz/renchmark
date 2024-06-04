use renchmark_utils::parse_config;

fn main() {
    println!("{}", parse_config("Hello Renchmark 👋!".to_string()));

    // 1. Parse config file
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
