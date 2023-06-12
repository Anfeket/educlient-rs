use educlient::Educlient;
use std::{fs::File, io::Write};

fn main() {
    // argv
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <username> <password> <domain>", args[0]);
        return;
    }
    let username = args[1].clone();
    let password = args[2].clone();
    let domain = args[3].clone();

    // login
    println!("Logging in");
    let mut client = Educlient::new(domain);
    let res = client.login(username, password);
    if res.is_err() {
        println!("Login failed");
        return;
    }
    println!("Login successful");

    //log
    println!("Logging data");
    let data = &client.data;
    let data = serde_json::to_string_pretty(data);
    if data.is_err() {
        println!("Failed to log data");
        return;
    }
    let data = data.unwrap();
    let file = File::create("data.json");
    if file.is_err() {
        println!("Failed to log data");
        return;
    }
    let mut file = file.unwrap();
    let res = file.write_all(data.as_bytes());
    if res.is_err() {
        println!("Failed to log data");
        return;
    }
    println!("Data logged");

    println!("Deserializing data");
    let deserialize = client.deserialize();
    if deserialize.is_err() {
        println!("Failed to deserialize data");
        return;
    }
    println!("Deserialization successful");

    let data = deserialize.unwrap();
    println!("{:?}", data.userdata);
}
