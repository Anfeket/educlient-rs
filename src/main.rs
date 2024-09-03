use educlient::Educlient;
use std::{
    fs::File,
    io::{Read, Write},
};

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
        }
        _ => {
            println!("Usage:\n  edupage-rs login <username> <password> <domain> [-j <file>]\n  edupage-rs import <file>");
            return;
        }
    };

    println!("Deserializing data...");
    let time = std::time::Instant::now();
    let data = client.data().unwrap();
    println!("Deserialized data in {}ms", time.elapsed().as_millis());

    println!("Select Date:");
    for (i, day_plan) in data.day_plans.iter().enumerate() {
        println!("{}. {}", i + 1, day_plan.date);
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>();
    if input.is_err() {
        println!("Invalid input");
        return;
    }
    let input = input.unwrap();
    if input > data.day_plans.len() {
        println!("Invalid input");
        return;
    }
    let day_plan = &data.day_plans[input - 1];
    println!("Lessons:");
    for lesson in &day_plan.lessons {
        if lesson.subject_id.is_none() {
            continue;
        }

        // Try to get the subject, use a placeholder if not found
        let subject = match data.dbi.subject_from_id(lesson.subject_id.unwrap()) {
            Some(subject) => subject,
            None => {
                println!(
                    "{}. {} - Subject not found",
                    lesson.period, "Unknown Teacher"
                );
                continue;
            }
        };

        // Try to get the plan, use a placeholder if not found
        let plan = match lesson.plan_id {
            Some(plan_id) => match data.dbi.plan_from_id(plan_id) {
                Some(plan) => plan,
                None => {
                    println!(
                        "{}. {} - {}",
                        lesson.period, "Unknown Teacher", subject.name
                    );
                    continue;
                }
            },
            None => {
                println!(
                    "{}. {} - {}",
                    lesson.period, "Unknown Teacher", subject.name
                );
                continue;
            }
        };

        // Try to get the first teacher, use a placeholder if not found
        let teacher_name = match plan.teachers.first() {
            Some(teacher_id) => match data.dbi.teacher_from_id(*teacher_id) {
                Some(teacher) => teacher.name(),
                None => "Unknown Teacher".to_string(),
            },
            None => "Unknown Teacher".to_string(),
        };

        // Print the lesson details
        println!("{}. {} - {}", lesson.period, teacher_name, subject.name);
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

fn login(args: &[String]) -> Option<Educlient> {
    if args.get(5) == Some(&"-j".to_string()) {
        let mut client = Educlient::new(args[4].to_string());
        if client
            .login(args[2].to_string(), args[3].to_string())
            .is_err()
        {
            println!("Failed to login");
            return None;
        }
        println!("Logged in as {}", args[4]);
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
    if client
        .login(args[2].to_string(), args[3].to_string())
        .is_err()
    {
        println!("Failed to login");
        return None;
    }
    Some(client)
}
