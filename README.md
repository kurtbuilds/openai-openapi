# `openai-openapi`

```rust
use openai_openapi::OpenAiClient;

#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let res = client.list_engines().await.unwrap();
    println!("{:#?}", res);
}
```

`openai-openapi` is a feature-complete, human, async client library for the OpenAI API, generated from OpenAI's OpenAPI spec using [`openapi-client-generator`](https://github.com/kurtbuilds/openapi-client-generator).

# Contributing

Contributions are welcome!