#![allow(dead_code)]

use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use toml;

pub type DepsSet = BTreeMap<String, Dependency>;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
struct CargoConfig {
    package: Package,
    dependencies: Option<DepsSet>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DependencyDetail),
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Dependency::Detailed(d) => d.version.as_deref().unwrap(),
                Dependency::Simple(d) => d,
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DependencyDetail {
    pub version: Option<String>,
    pub registry: Option<String>,
    pub registry_index: Option<String>,
    pub path: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub optional: bool,
    pub default_features: Option<bool>,
    pub package: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: y-cargo-deps <toml file>");
        return;
    }
    let input_file = args[1].clone();
    let file = File::open(input_file);
    match file {
        Ok(mut f) => {
            let mut file_content = String::new();
            let _bytes_read = f.read_to_string(&mut file_content).unwrap();
            let decoded_file: CargoConfig = toml::from_str(&file_content).unwrap();
            let ref deps = decoded_file.dependencies.unwrap();
            let mut res = String::new();
            for (key, value) in deps.iter() {
                res.push_str(format!("{}:{}\n", key, value).as_str());
            }
            print!("{}", res);
        }
        Err(e) => println!("{}", e),
    }
}
