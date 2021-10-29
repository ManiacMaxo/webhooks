use crate::utils::cli::args;
use crate::utils::config::{load_config, Config, RuleSources};
use hyper::{Body, Request, Response, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;

pub async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let options = args();
    let config = load_config(options.config_file).unwrap();
    let mut response = Response::new(Body::empty());

    match config.get(&req.uri().path()[1..]) {
        Some(conf) => {
            if check_auth(req, conf) {
                *response.status_mut() = StatusCode::OK;
            } else {
                *response.status_mut() = StatusCode::UNAUTHORIZED;
            }
        }
        None => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    return Ok(response);
}

fn check_auth(req: Request<Body>, config: &Config) -> bool {
    if config.auth_rule.is_none() {
        return true;
    }

    let auth_rule = config.auth_rule.as_ref().unwrap();
    let param = auth_rule.parameter.as_ref().unwrap();

    match param.source {
        RuleSources::header => {
            let header = req.headers().get(param.name.as_str().unwrap());
            if header.is_some() && header.unwrap() == &auth_rule.value {
                return true;
            }
            return false;
        }
        RuleSources::url => {
            let query = parse_query(req.uri().query().unwrap());

            println!("{:?}", query);

            match query.get(param.name.as_str().unwrap()) {
                Some(value) => value == &auth_rule.value,
                None => false,
            }
        }
        RuleSources::body => return true, // Unimplemented
    }
}

fn parse_query(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for val in query.split('&') {
        let mut pair = val.split('=');
        map.insert(
            pair.next().unwrap().to_string(),
            pair.next().unwrap().to_string(),
        );
    }

    return map;
}

pub async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
