use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let http_client = reqwest::ClientBuilder::default()
        .connect_timeout(Duration::from_millis(100))
        .timeout(Duration::from_millis(1000))
        .connection_verbose(true)
        .build()
        .expect("Successful client build");

    let http_req = http_client
        .get("http://httpstat.us/200")
        .build()
        .expect("Successful HTTP Request creation");

    let http_resp = http_client
        .execute(http_req)
        .await
        .expect("Successful HTTP Response");

    println!("HTTP RESP Result : {:?}", http_resp);

    let http_resp_body = http_resp
        .text()
        .await
        .expect("Successful HTTP Response Body");

    println!("HTTP RESP : {}", http_resp_body);
}
