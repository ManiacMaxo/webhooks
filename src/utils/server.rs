use crate::utils::algorithms::verify_signature;
use crate::utils::cli::args;
use crate::utils::config::{load_config, Config, RuleSources, RuleTypes};
use hyper::body;
use hyper::{Body, Request, Response, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;

macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

pub async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let options = args();
    let config = load_config(options.config_file).unwrap();
    let mut response = Response::new(Body::empty());

    match config.get(&req.uri().path()[1..]) {
        Some(conf) => {
            *response.status_mut() = check_auth(req, conf).await;

            if response.status() == StatusCode::OK {
                *response.body_mut() = Body::from("Ok");
                exec_command(conf);
            }
        }
        None => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    *response.body_mut() = match response.status() {
        StatusCode::OK => Body::from("Ok"),
        StatusCode::UNAUTHORIZED => Body::from("Unauthorized"),
        StatusCode::NOT_FOUND => Body::from("No matching hook found"),
        _ => Body::from(response.status().canonical_reason().unwrap()),
    };

    Ok(response)
}

async fn check_auth(req: Request<Body>, config: &Config) -> StatusCode {
    if config.auth_rule.is_none() {
        return StatusCode::NOT_FOUND;
    }

    let auth_rule = config.auth_rule.as_ref().unwrap();

    match auth_rule.r#type {
        RuleTypes::value => {
            if auth_rule.parameter.is_none() {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }

            let param = auth_rule.parameter.as_ref().unwrap();
            match param.source {
                RuleSources::header => {
                    either!(
                        check_headers(req, param.name.as_str().unwrap(), &auth_rule.value) => StatusCode::OK;
                        StatusCode::UNAUTHORIZED
                    )
                }
                RuleSources::url => {
                    either!(
                        check_url(req, param.name.as_str().unwrap(), &auth_rule.value) => StatusCode::OK;
                        StatusCode::UNAUTHORIZED
                    )
                }
                RuleSources::body => StatusCode::OK, // Not Implemented
            }
        }
        RuleTypes::github => {
            const GITHUB_AUTH_HEADER: &str = "X-Hub-Signature-256";

            match req.headers().get(GITHUB_AUTH_HEADER) {
                Some(header) => {
                    // let payload = read_request_body(req).await.unwrap();
                    let signature = &header.to_str().unwrap();
                    either!(verify_signature(
                        b"some body",
                        signature,
                        "hmacSha256",
                        Some(auth_rule.value.as_str()),
                    ) => StatusCode::OK; StatusCode::UNAUTHORIZED)
                }
                None => return StatusCode::UNAUTHORIZED,
            }
        }
        RuleTypes::bitbucket => StatusCode::OK, // Not Implemented
        RuleTypes::ip => StatusCode::OK,        // Not Implemented
    }
}

async fn read_request_body(req: Request<Body>) -> Result<String, hyper::Error> {
    let bytes = body::to_bytes(req.into_body()).await?;
    Ok(String::from_utf8(bytes.to_vec()).expect("request was not valid utf-8"))
}

fn check_headers(req: Request<Body>, key: &str, value: &str) -> bool {
    match req.headers().get(key) {
        Some(header) => header.to_str().unwrap() == value,
        None => false,
    }
}

fn check_url(req: Request<Body>, key: &str, value: &str) -> bool {
    if req.uri().query().is_none() {
        return false;
    }

    let query = parse_query(req.uri().query().unwrap());
    println!("{:?}", query);
    match query.get(key) {
        Some(v) => v == value,
        None => true,
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

fn exec_command(config: &Config) {
    let command = &config.command;

    println!("{:?}", command);
}

pub async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
