extern crate reqwest;

use std::thread;
use std::time;

fn main() {
    let client = reqwest::Client::new();
    println!("First request...");
    let first = client.get("http://security.ubuntu.com/ubuntu/dists/xenial/Release.gpg").send().unwrap();
    println!("Done! {:?}. Sleeping for six seconds.", first.status());
    thread::sleep(time::Duration::from_millis(6000));
    println!("Done! Second request...");
    client.get("http://security.ubuntu.com/ubuntu/dists/xenial/Release.gpg").send().unwrap();
}

