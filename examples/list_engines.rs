use openai_openapi::OpenAiClient;

#[tokio::main]
async fn main() {
    let client = OpenAiClient::from_env();
    let res = client.list_engines().await.unwrap();
    println!("{:#?}", res);
}