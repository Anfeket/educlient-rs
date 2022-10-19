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

    // create client
    let client = Educlient::new(&domain);

    // login
    client.login(username, password);
    if client.logged_in {
        println!("Logged in!");
    } else {
        panic!("Login failed!");
    }

    // get grades
    let grades = client.get_grades().1;
    println!("{}", grades);
    
}