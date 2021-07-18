use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

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
    pub name: String,
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
pub struct Config {
    pub name: String,
    pub command: String,
    pub working_directory: String,
    pub auth_rule: Option<Rule>,
}

pub fn load_config(path: String) -> Result<Vec<Config>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: Vec<Config> = from_reader(reader)?;

    for config_item in &config {
        let auth_rule = config_item.auth_rule.as_ref().unwrap();

        match auth_rule.r#type {
            RuleTypes::value => {
                if auth_rule.parameter.is_none() {
                    return Err(Error::new(ErrorKind::NotFound, "No parameter for rule"));
                }
            }
            _ => {}
        }
    }
    return Ok(config);
}
