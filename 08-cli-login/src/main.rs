use sha2::{Digest, Sha512};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

const USER_DB_FILE: &str = "./users.json";

fn main() {
    let mut file = open_file(USER_DB_FILE);
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parsed = json::parse(&contents).unwrap();

    let mut users = parse_users(&parsed).expect("user database is invalid");

    loop {
        let line = prompt("please specify an action: [l]ogin / [a]dd user / [q]uit> ");

        match line.chars().next() {
            Some('l') => login(&users),
            Some('a') => {
                add_user(&mut users);

                let mut file = open_file(USER_DB_FILE);
                write_user_db(&mut file, &users);
            }
            Some('q') => return,
            _ => println!("invalid action \"{}\"", line),
        };
    }
}

fn login(users: &Vec<User>) {
    let username = prompt("username> ");
    let password = prompt("password> ");

    for user in users {
        if user.username == username && sha512(&password) == user.hash {
            println!("login success");
            return;
        }
    }

    println!("login failed");
}

fn add_user(users: &mut Vec<User>) {
    let username = prompt("username> ");
    let password = prompt("password> ");

    let username_exists = users.iter().any(|u| u.username == username);

    if username_exists {
        println!("user \"{}\" already exists", username);
        return;
    }

    users.push(User {
        username,
        hash: sha512(&password),
    });

    println!("user added successfully");
}

fn open_file(filename: &str) -> File {
    let path = Path::new(filename);

    if !path.exists() {
        println!("Creating \"{}\" ...", filename);

        let mut file = File::create(&path).unwrap();
        file.write_all(b"[]").unwrap();
    }

    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
        .unwrap()
}

struct User {
    username: String,
    hash: String,
}

fn parse_users(parsed: &json::JsonValue) -> Option<Vec<User>> {
    match parsed {
        json::JsonValue::Array(users) => Some(
            users
                .iter()
                .map(|user| User {
                    username: user["username"].to_string(),
                    hash: user["hash"].to_string(),
                })
                .collect(),
        ),
        _ => None,
    }
}

fn prompt(p: &str) -> String {
    print!("{}", p);
    std::io::stdout().flush().ok().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();

    line
}

fn sha512(plain: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(plain.as_bytes());

    format!("{:X}", hasher.finalize())
}

fn write_user_db(file: &mut File, users: &[User]) {
    let mut tmp = json::JsonValue::new_array();

    users.iter().for_each(|user| {
        tmp.push(json::object! {
            username: user.username.clone(),
            hash: user.hash.clone(),
        }).unwrap();
    });

    file.write_all(tmp.dump().as_bytes()).unwrap();
}
