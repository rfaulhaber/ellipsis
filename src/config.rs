use anyhow::Result;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

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

#[derive(Deserialize, Error, Debug)]
pub enum VariableResolutionError {
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
}

impl Config {
    /// Resolves a string used in the config with a variable.
    /// If the string contains a variable, such as "{{foo}}/exec.sh", this function resolves "{{foo}}", creating a new string.
    pub fn resolve_variable_string(&self, text: String) -> Result<String> {
        let matching_bracket_regex = Regex::new("\\{{2}([^}]\\w*)\\}{2}").unwrap();

        let matches = matching_bracket_regex
            .captures_iter(&text)
            .zip(matching_bracket_regex.find_iter(&text));

        let mut new_str = text.clone();

        for (c, f) in matches {
            let var = match self.vars.get(&c[1]) {
                Some(v) => v,
                None => {
                    return Err(
                        VariableResolutionError::UndefinedVariable(String::from(&c[1])).into(),
                    )
                }
            };

            new_str.replace_range(f.range(), var);
        }

        Ok(new_str)
    }
}
