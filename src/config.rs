use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub vars: HashMap<String, String>,
    pub hosts: HashMap<String, ConfigHost>,
    pub tasks: HashMap<String, LiteralTaskDefinition>,
    pub links: Vec<LinkDefinition>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TaskDefinition {
    Ref(String),
    Literal(LiteralTaskDefinition),
}

#[derive(Deserialize, Debug)]
pub struct LiteralTaskDefinition {
    pub name: Option<String>,
    pub exec: String,
    pub revert: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigHost {
    pub tasks: Vec<TaskDefinition>,
    pub links: Vec<LinkDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct LinkDefinition {
    pub from: String,
    pub to: String,
}
