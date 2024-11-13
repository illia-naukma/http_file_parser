use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fmt::Display;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct HttpParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    PestError(#[from] Box<PestError<Rule>>),

    #[error("Unknown HTTP method: {0}")]
    UnknownHttpMethod(String),
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
        })
    }
}

impl HttpMethod {
    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            _ => Err(ParseError::UnknownHttpMethod(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    fn parse(pair: Pair<Rule>) -> Result<Self, ParseError> {
        let mut inner = pair.into_inner();
        let method = HttpMethod::from_str(inner.next().unwrap().as_str())?;
        let url = inner.next().unwrap().as_str().to_string();
        let version = inner.next().unwrap().as_str().to_string();
        let mut headers = HashMap::new();
        let mut body = String::new();

        for item in inner {
            match item.as_rule() {
                Rule::headers => {
                    for header in item.into_inner() {
                        let mut kv = header.into_inner();
                        headers.insert(
                            kv.next().unwrap().as_str().to_string(),
                            kv.next().unwrap().as_str().to_string(),
                        );
                    }
                }
                Rule::body => body = item.as_str().trim().to_string(),
                _ => (),
            }
        }

        Ok(HttpRequest {
            method,
            url,
            version,
            headers,
            body,
        })
    }
}

#[derive(Debug)]
pub struct HttpFile {
    pub requests: Vec<HttpRequest>,
}

impl HttpFile {
    fn parse(pair: Pair<Rule>) -> Result<Self, ParseError> {
        let requests = pair
            .into_inner()
            .filter(|item| item.as_rule() == Rule::request)
            .map(HttpRequest::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(HttpFile { requests })
    }
}

pub fn parse(input: &str) -> Result<HttpFile, ParseError> {
    let file = HttpParser::parse(Rule::http_file, input.trim_start())
        .expect("unable to parse")
        .next()
        .unwrap();

    HttpFile::parse(file)
}
