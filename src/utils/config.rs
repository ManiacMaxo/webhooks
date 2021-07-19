use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_json::Value;
use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum RuleTypes {
    value,
    ip,
    github,    // X-Hub-Signature-256 HMACSha256
    bitbucket, // X-Hub-Signature HMACSha256
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum RuleSources {
    header,
    url,
    body,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub name: Value,
    pub source: RuleSources,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Rule {
    pub r#type: RuleTypes,
    pub value: String,
    pub parameter: Option<Parameter>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Json {
    pub name: String,
    pub command: String,
    pub working_directory: String,
    pub auth_rule: Option<Rule>,
}

#[derive(Debug)]
pub struct Config {
    pub command: String,
    pub working_directory: String,
    pub auth_rule: Option<Rule>,
}

pub type ConfigMap = HashMap<String, Config>;

pub fn load_config(path: String) -> Result<ConfigMap, &'static str> {
    let file = File::open(path).expect("Cannot open config file");
    let reader = BufReader::new(file);
    let json: Vec<Json> = from_reader(reader).expect("Cannot parse JSON");
    let mut config: ConfigMap = HashMap::new();

    for config_item in json {
        let auth_rule = config_item.auth_rule.as_ref().unwrap();

        match auth_rule.r#type {
            RuleTypes::value => {
                if auth_rule.parameter.is_none() {
                    return Err("Some error message");
                }
            }
            _ => {}
        }
        config.insert(
            config_item.name,
            Config {
                command: config_item.command,
                working_directory: config_item.working_directory,
                auth_rule: config_item.auth_rule,
            },
        );
    }

    return Ok(config);
}
