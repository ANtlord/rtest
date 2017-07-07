use std::fs::File;
use std::io::{Write, stdin, BufReader, BufRead, Stdin};
use std::str::FromStr;

pub struct Auth {
    pub user: String,
    pub pass: String,
    pub host: String,
    pub port: u16,
}

impl Auth {
    pub fn new(filename: &str) -> Self {
        let (login, password, host, port) = match File::open(filename) {
            Ok(file) => { get_auth(&file) },
            Err(_) => { make_auth_file(&filename) }
        };
        Self { user: login, pass: password, host: host, port: port }
    }
}

struct AuthReader {
    buffer: String,
    stdin: Stdin,
    file: File,
}

impl AuthReader {
    fn new(filename: &str) -> Self {
        Self {
            buffer: String::new(), stdin: stdin(),
            file: File::create(&filename).unwrap()
        }
    }

    fn from_user(&mut self, option_name: &str) -> String {
        self.buffer.clear();
        println!("{}:", &option_name);
        self.stdin.read_line(&mut self.buffer).unwrap();
        self.file.write(self.buffer.as_bytes()).unwrap();
        let result = String::from_str(&self.buffer[0 .. self.buffer.len() - 1]).unwrap();
        return result;
    }
}

pub fn make_auth_file(filename: &str) -> (String, String, String, u16) {
    let mut reader = AuthReader::new(filename);
    let user = reader.from_user("Login");
    let pass = reader.from_user("Password");
    let host = reader.from_user("Address");
    let port = reader.from_user("Port").parse::<u16>().unwrap();
    (user, pass, host, port)
}

pub fn get_auth(from_file: &File) -> (String, String, String, u16) {
    let mut reader = BufReader::new(from_file);
    let mut user = String::new();
    let mut pass = String::new();
    let mut host = String::new();
    let mut port = String::new();
    reader.read_line(&mut user).expect("cannot read the a user from the file");
    reader.read_line(&mut pass).expect("cannot read a pass from the file");
    reader.read_line(&mut host).expect("cannot read a host from the file");
    reader.read_line(&mut port).expect("cannot read a port from the file");

    user.pop().unwrap();
    pass.pop().unwrap();
    host.pop().unwrap();
    port.pop().unwrap();
    let port = port.parse::<u16>().unwrap();

    (user, pass, host, port)
}
