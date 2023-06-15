use educlient::Educlient;
use std::{fs::File, io::{Write, Read}};

fn main() {
    // argv
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage:\n  edupage-rs login <username> <password> <domain> [-j <file>]\n  edupage-rs import <file>");
        return;
    }
    let client = match args.get(1).unwrap().as_str() {
        "login" => login(&args).unwrap(),
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
            println!("Usage:\n  edupage-rs login <username> <password> <domain> [-j <file>]\n  edupage-rs import <file>");
            return;
        },
    };

    println!("Deserializing data");
    let time = std::time::Instant::now();
    let deserialize = client.deserialize();
    if deserialize.is_err() {
        println!("Failed to deserialize data");
        return;
    }
    println!("Deserialization successful, took {}ms", time.elapsed().as_millis());
    let data = deserialize.unwrap();

    println!("Select Date:");
    let mut i = 0;
    for date in data.day_plans.keys() {
        println!("{}: {}", i, date);
        i += 1;
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>();
    if input.is_err() {
        println!("Invalid input");
        return;
    }
    let input = input.unwrap();
    let date = data.day_plans.keys().nth(input);
    if date.is_none() {
        println!("Invalid input");
        return;
    }
    let date = date.unwrap();
    let lessons = data.day_plans.get(date);
    if lessons.is_none() {
        println!("Invalid input");
        return;
    }
    let lessons = lessons.unwrap();

    println!("Lessons for {}:", date);
    for lesson in lessons {
        if lesson.subject_id.is_none() {
            println!("{}: {}", lesson.period, "Free");
            continue;
        }
        let name = &data.dbi.subjects[&lesson.subject_id.unwrap()].name;
        println!("{}: {}", lesson.period, name);
    }

    println!("\nLatest messages: ");
    let mut c = 0;
    for message in data.timeline {
        if message.text.is_empty() {
            continue;
        }
        if c > 5 {
            break;
        }
        println!("{}: {}", message.author, message.text);
        c += 1;
    }
}

fn login(args: &Vec<String>) -> Option<Educlient> {
    if args.get(5) == Some(&"-j".to_string()) {
        let mut client = Educlient::new(args[4].to_string());
        if client.login(args[2].to_string(), args[3].to_string()).is_err() {
            println!("Failed to login");
            return None;
        } else {
            println!("Logged in as {}", args[4]);
        }
        let file = File::create(&args[6]);
        if file.is_err() {
            println!("Failed to create file");
        }
        let mut file = file.unwrap();
        let data = serde_json::to_string_pretty(&client.json).unwrap();
        let res = file.write_all(data.as_bytes());
        if res.is_err() {
            println!("Failed to log data");
        } else {
            println!("Logged data to {}", args[6]);
        }
        return Some(client);
    }
    let mut client = Educlient::new(args[4].to_string());
    if client.login(args[2].to_string(), args[3].to_string()).is_err() {
        println!("Failed to login");
        return None;
    }
    return Some(client);
}
