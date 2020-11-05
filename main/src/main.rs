use std::time::Duration;

use reqwest::blocking::ClientBuilder;

fn main() {
    println!("Hello, world!");

    let http_client = ClientBuilder::default()
        .connect_timeout(Some(Duration::from_millis(100)))
        .timeout(Some(Duration::from_millis(1000)))
        .connection_verbose(true)
        .build()
        .expect("Successful client build");

    let http_req = http_client
        .get("http://httpstat.us/200")
        .build()
        .expect("Successful HTTP Request creation");

    let http_resp = http_client
        .execute(http_req)
        .expect("Successful HTTP Response");

    println!("HTTP RESP Result : {:?}", http_resp);

    let http_resp_body = http_resp.text().expect("Successful HTTP Response Body");

    println!("HTTP RESP : {}", http_resp_body);
}
