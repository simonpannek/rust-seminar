use serde::Deserialize;
use warp::Filter;

const ADDRESS: ([u8; 4], u16) = ([127, 0, 0, 1], 8080);

#[derive(Deserialize, Debug)]
struct Data {
    number: u32,
    boolean: bool,
}

#[tokio::main]
async fn main() {
    let filter = warp::path!("data")
        .and(warp::post())
        .and(warp::body::json())
        .map(|data: Data| format!("Data received: {:?}", data));

    warp::serve(filter).run(ADDRESS).await;
}
