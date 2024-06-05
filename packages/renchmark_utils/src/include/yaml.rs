use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RenchmarkConfig {
    renchmark: RenchmarkYaml,
    globals: Globals,
    flows: Flows,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Flows {
    base_url: Option<String>,
    before_all: Option<Box<Flows>>,
    after_all: Option<Box<Flows>>,
    url: Option<String>,
    uri: Option<String>,
    method: Option<String>,//Option<Method>,
    data: Option<HashMap<String, String>>,
    auth_from: Option<String>,
    headers: Option<Vec<String>>,
    flows: Option<Box<Flows>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Globals {
    base_url: Option<String>,
    globals: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RenchmarkYaml {
    generate_file: Option<String>,
    generate_html: Option<bool>,
    show_summary: Option<bool>,
    users: Option<Users>,
    app_nodes: Vec<Nodes>,
    orchestrate_nodes: Vec<Nodes>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Nodes {
    auth_type: AuthType,
    is_docker: bool,
    nodes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Users {
    total: u64,
    concurrent: u64,
    speed: u32,
    fail_less_than: Option<u64>,
    fail_after: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum AuthType {
    SSH,
    UsernameAndPassword(String, String),
}