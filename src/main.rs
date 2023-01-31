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
    println!("Logging in");
    let mut client = Educlient::new(domain);
    let res = client.login(username, password);
    if res.is_err() {
        println!("Login failed");
        return;
    }
    println!("Login successful");

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
