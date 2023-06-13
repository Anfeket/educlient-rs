use educlient::Educlient;
use std::{fs::File, io::{Write, Read}};

fn main() {
    // argv
    let args: Vec<String> = std::env::args().collect();
    let client = match args.get(1).unwrap().as_str() {
        "login" => login(&args[2], &args[3], &args[4]).unwrap(),
        "import" => {
            let mut client = Educlient::new("".to_string());
            let file = File::open(&args[2]);
            if file.is_err() {
                println!("Failed to open file");
                return;
            }
            let mut file = file.unwrap();
            let mut data = String::new();
            let res = file.read_to_string(&mut data);
            if res.is_err() {
                println!("Failed to read file");
                return;
            }
            client.json = serde_json::from_str(&data).unwrap();
            client
        },
        _ => {
            println!("Usage: edupage-rs <login|import> <domain> <username> <password> | <file>");
            return;
        },
    };

    //log
    println!("Logging data");
    let data = &client.json;
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
    let time = std::time::Instant::now();
    let deserialize = client.deserialize();
    if deserialize.is_err() {
        println!("Failed to deserialize data");
        return;
    }
    println!("Deserialization successful");
    println!("Deserialization took {}ms", time.elapsed().as_millis());

    let data = deserialize.unwrap();
    println!("{:?}", data.userdata);
}

fn login(username:&String, password:&String, domain:&String) -> Option<Educlient> {
    println!("Logging in");
    let mut client = Educlient::new(domain.to_string());
    let res = client.login(username.to_string(), password.to_string());
    if res.is_err() {
        println!("Login failed");
        return None;
    }
    println!("Login successful");
    Some(client)
}
