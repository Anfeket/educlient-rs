use educlient::Educlient;

fn main() {
    // user input
    let mut username = String::new();
    let mut password = String::new();
    let mut domain = String::new();
    println!("Username:");
    std::io::stdin().read_line(&mut username).expect("Failed to read line");
    println!("Password:");
    std::io::stdin().read_line(&mut password).expect("Failed to read line");
    println!("Domain:");
    std::io::stdin().read_line(&mut domain).expect("Failed to read line");

    let mut client = Educlient::new(username, password, domain);
    client.login();
    let (_client, grades) = client.get_grades();
    println!("{:?}", grades);
}