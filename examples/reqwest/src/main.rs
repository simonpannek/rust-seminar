use serde::Serialize;

const URL: &str = "http://127.0.0.1:8080/data";

#[derive(Serialize)]
struct Data {
    number: u32,
    boolean: bool,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let response = client
        .post(URL)
        .json(&Data {
            number: 5,
            boolean: true,
        })
        .send()
        .await
        .expect("Failed to send post request.");

    println!(
        "{}",
        response
            .text()
            .await
            .expect("Failed to get text from response.")
    );
}
