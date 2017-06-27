use std::fs::{File};
use std::io::{Write, stdin, BufReader, BufRead};
use std::str::FromStr;

fn make_auth_file(filename: &str) -> (String, String) {
    let mut buffer = String::new();
    let stdin = stdin();
    let mut file = File::create(filename).unwrap();

    println!("Login:");
    stdin.read_line(&mut buffer).unwrap();
    file.write(buffer.as_bytes()).unwrap();
    let login = String::from_str(&buffer[0 .. buffer.len() - 1]).unwrap();

    buffer.clear();

    println!("Password:");
    stdin.read_line(&mut buffer).expect("cannot read a password");
    let password = String::from_str(&buffer[0 .. buffer.len() - 1]).unwrap();
    file.write(buffer.as_bytes()).expect("cannot write a password");

    (login, password)
}

fn get_auth(from_file: &File) -> (String, String) {
    let mut reader = BufReader::new(from_file);
    let mut login = String::new();
    let mut password = String::new();
    reader.read_line(&mut login).expect("cannot read the a login from the file");
    reader.read_line(&mut password).expect("cannot read a password from the file");

    login.pop().unwrap();
    password.pop().unwrap();
    (login, password)
}

pub struct Auth {
    pub login: String,
    pub password: String,
}

impl Auth {
    pub fn new(filename: &str) -> Self {
        let (login, password) = match File::open(filename) {
            Ok(file) => { get_auth(&file) },
            Err(_) => { make_auth_file(&filename) }
        };
        Self { login: login, password: password }
    }
}
