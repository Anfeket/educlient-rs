use educlient::Educlient;

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
    let mut client = Educlient::new(domain);
    let res = client.login(username, password);
    if res.is_err() {
        println!("Login failed");
        return;
    }

    let deserialize = client.deserialize();
    if deserialize.is_err() {
        println!("Failed to deserialize data");
        return;
    }

    let data = deserialize.unwrap();
    //write to file
    let file = std::fs::File::create("data.json").unwrap();
    serde_json::to_writer_pretty(&file, &data).unwrap();
}