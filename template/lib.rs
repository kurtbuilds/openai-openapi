//! This library is generated by [`openapi-client-generator`](https://crates.io/crates/openapi-client-generator).
//!
//! The [`OpenAiClient`] is the main entry point for this library.
#![allow(non_camel_case_types)]

use serde_json::json;
use httpclient::RequestBuilder;
pub mod model;
use crate::model::*;

pub struct OpenAiAuthentication {
    pub api_key: String,
}

impl OpenAiAuthentication {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn from_env() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        Self::new(api_key)
    }
}

impl OpenAiClient {
    pub fn from_env() -> Self {
        Self::new("https://api.openai.com/v1")
            .with_authentication(OpenAiAuthentication::from_env())
    }
}

trait Authenticatable {
    fn authenticate(self, authenticator: &Option<OpenAiAuthentication>) -> Self;
}

impl<'a> Authenticatable for RequestBuilder<'a> {
    fn authenticate(self, authenticator: &Option<OpenAiAuthentication>) -> Self {
        if let Some(authenticator) = authenticator {
            self
                .header("Authorization", &format!("Bearer {}", authenticator.api_key))
        } else {
            self
        }
    }
}