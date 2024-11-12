use crate::parser::{HttpMethod, HttpRequest};
use reqwest::blocking::{Client, Response};
use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] ReqwestError),
}

pub fn make_request(request: &HttpRequest) -> Result<Response, RequestError> {
    let client = Client::new();
    let url = request.url.to_string();

    let response = match request.method {
        HttpMethod::Get => client.get(&url).send()?,
        HttpMethod::Post => client.post(&url).body(request.body.clone()).send()?,
        HttpMethod::Put => client.put(&url).body(request.body.clone()).send()?,
        HttpMethod::Delete => client.delete(&url).send()?,
    };

    Ok(response)
}
