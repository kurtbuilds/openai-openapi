<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/openai-openapi/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/openai-openapi.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/openai-openapi/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/openai-openapi.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/openai-openapi/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/openai-openapi/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/openai-openapi">
    <img src="https://img.shields.io/crates/d/openai-openapi?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/openai-openapi">
    <img src="https://img.shields.io/crates/v/openai-openapi?style=flat-square" alt="Crates.io" />
</a>

</p>


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

`openai-openapi` is a feature-complete, human, async client library for the OpenAI API, generated from OpenAI's OpenAPI spec using [`openai-openapi`](https://github.com/kurtbuilds/openai-openapi).

# Contributing

Contributions are welcome!