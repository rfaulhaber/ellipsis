use anyhow::Result;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub vars: HashMap<String, String>,
    pub hosts: HashMap<String, ConfigHost>,
    pub tasks: HashMap<String, LiteralTaskDefinition>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TaskDefinition {
    Ref(String),
    Literal(LiteralTaskDefinition),
}

#[derive(Deserialize, Debug, Clone)]
pub struct LiteralTaskDefinition {
    pub name: Option<String>,
    pub exec: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigHost {
    pub tasks: Vec<TaskDefinition>,
    pub links: Vec<LinkDefinition>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LinkKind {
    Hard,
    Soft,
    Copy,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LinkDefinition {
    pub from: String,
    pub to: String,
    pub name: Option<String>,
    pub kind: Option<LinkKind>,
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

        let mut new_str = text.clone();
        let mut new_str_im = new_str.clone();

        while matching_bracket_regex.is_match(&new_str_im) {
            let (c, f) = matching_bracket_regex
                .captures_iter(&new_str_im)
                .zip(matching_bracket_regex.find_iter(&new_str_im))
                .next()
                .unwrap();

            let var = match self.vars.get(&c[1]) {
                Some(v) => v,
                None => {
                    return Err(
                        VariableResolutionError::UndefinedVariable(String::from(&c[1])).into(),
                    )
                }
            };

            new_str.replace_range(f.range(), var);
            new_str_im = new_str.clone();
        }

        Ok(new_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn resolve_variable_string_replaces_correctly() {
        let mut config_vars: HashMap<String, String> = HashMap::new();
        config_vars.insert("foo".into(), "/usr/bin/foo".into());
        config_vars.insert("bar".into(), "/home/bar/.config".into());
        config_vars.insert("baz".into(), "brew install cargo".into());

        let config = Config {
            vars: config_vars,
            hosts: HashMap::new(),
            tasks: HashMap::new(),
        };

        assert_eq!(
            config
                .resolve_variable_string("{{foo}}/bar/baz".into())
                .unwrap(),
            "/usr/bin/foo/bar/baz"
        );

        assert_eq!(
            config
                .resolve_variable_string("{{foo}}/qqq{{bar}}".into())
                .unwrap(),
            "/usr/bin/foo/qqq/home/bar/.config"
        );
    }
}
