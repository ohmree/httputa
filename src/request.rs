//! HTML request types
use crate::common::{HttpError, HttpVersion};
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf, str::FromStr};
use strum_macros::{Display, EnumString};

/// Standard HTML request methods
#[derive(Debug, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Options,
    Connect,
    Patch,
}

#[derive(Debug)]
pub struct RequestMessage {
    pub method: Method,
    pub path: PathBuf,
    pub http_version: HttpVersion,
    // TODO: maybe have an enum for all the header fields?
    pub header_fields: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl RequestMessage {
    pub fn new<'a>(message: &'a str) -> Result<Self> {
        let mut lines = message.lines();

        // Request line
        // TODO: implement actual error types
        let request_line = lines
            .next()
            .ok_or(HttpError::FieldNotFound("request_line"))?;
        let mut words = request_line.split(' ');
        let method = Method::from_str(words.next().ok_or(HttpError::FieldNotFound("method"))?)?;
        let path = PathBuf::from_str(words.next().ok_or(HttpError::FieldNotFound("path"))?)?;
        let http_version = HttpVersion::from_str(
            words
                .next()
                .ok_or(HttpError::FieldNotFound("http_version"))?,
        )?;

        // TODO: actually populate this
        let header_fields = Some(HashMap::new());
        // lines
        //     .next()
        //     .ok_or(HttpError::FieldNotFound("header_fields"))?;

        // TODO: HUH?
        // Empty line
        // lines.next().ok_or(anyhow!("Error"))?;

        let body = Some(lines.collect::<String>());

        Ok(Self {
            method,
            path,
            http_version,
            header_fields,
            body,
        })
    }
}
