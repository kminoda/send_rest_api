use std::env;
use std::fs::File;
use std::io::Read;

use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: send_rest_api <json_path> <endpoint_url>");
        return;
    }

    let json_path = &args[1];
    let endpoint_url = &args[2];

    // JSONファイルを読み込む
    let mut file = File::open(json_path).expect("Failed to open JSON file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Failed to read JSON file");

    let json: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // REST APIリクエストを送信する
    let client = Client::new();
    let response = client
        .post(endpoint_url)
        .json(&json)
        .send()
        .await
        .expect("Failed to send request");

    // レスポンスを処理する
    if response.status().is_success() {
        let response_body = response.text().await.expect("Failed to read response body");
        println!("Response: {}", response_body);
    } else {
        println!("Request failed with status: {}", response.status());
    }
}