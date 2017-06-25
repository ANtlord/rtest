extern crate getopts;
extern crate imap;
extern crate openssl;
mod points;

use std::fs::{File, remove_file};
use std::str::FromStr;
use std::thread;
use std::sync::mpsc::TryRecvError;
use std::io::{Write, stdin, stdout, SeekFrom, Seek, BufReader, BufRead};
use std::ops::Deref;
use std::env;
use std::time;
use std::sync::mpsc;
use std::time::Duration;

use getopts::Options;
use imap::client::Client;
use openssl::ssl::{SslConnectorBuilder, SslMethod};

use points::*;

struct Angle {
    v1: Point3D,
    v2: Point3D,
}

trait HasCount : HasLen {
    fn count(&self) -> f32 {
        1.
    }
}

impl HasCount for Angle {
    fn count(&self) -> f32 {
        2.
    }       
}

impl<T> HasLen for T where T: HasCount {
    fn len(&self) -> f32 {
        self.count()
    }
}

fn gen<T: HasLen>(obj: &T) {
    println!("{}", obj.len());
}

fn modify(point: &mut Point3D) {
    point.x = 111.
}

struct Counter {
    val: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.val += 1;
        if self.val < 6 {
            Some(self.val)
        } else {
            None
        }
    }
}

impl Deref for Counter {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.val
    }
}

fn get_folder_content(login: String, password: String) {
    let (tx, rx) = mpsc::channel();
    indicate_loading(rx);

    // let login = login.clone();
    // let password = password.clone();

    let h = thread::spawn(move || {
        let host = "imap.gmail.com";
        let mut imap_socket = Client::secure_connect(
            (host, 993), host,
            SslConnectorBuilder::new(SslMethod::tls()).unwrap().build()
        ).unwrap();

        imap_socket.login(&login, &password).expect("cannot login");
        let mb = imap_socket.select("INBOX").unwrap();
        imap_socket.logout().unwrap();
        tx.send(true).unwrap();
        thread::sleep(Duration::from_millis(500));
        println!("\n{}", mb);
    });
    h.join().unwrap();
}

fn draw_loading(mut count: u8) -> u8 {
    thread::sleep(Duration::from_millis(500));
    if count % 3 == 0 {
        print!("\r   \r");
        count = 0;
    }
    print!(".");
    stdout().flush().unwrap();
    count + 1
}

fn indicate_loading(rx: mpsc::Receiver<bool>) {
    thread::spawn(move || {
        let mut count = 0;
        let mut is_done = false;
        while !is_done {
            count = draw_loading(count);
            match rx.try_recv() {
                 Ok(t)  => { is_done = t },
                 Err(TryRecvError::Disconnected) => { is_done = true },
                 Err(TryRecvError::Empty) => {}
            }
        }
    });
}

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("r", "erase", "erase auth file");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        println!("this is help message");
        return;
    }

    let filename = "auth.txt";
    if matches.opt_present("r") {
        match remove_file(filename) {
            Ok(_) => {  }
            Err(_) => {  }
        }
    }
    let (login, password) = match File::open(filename) {
        Ok(file) => { get_auth(&file) },
        Err(_) => { make_auth_file(&filename) }
    };

    println!("{}", &login);
    println!("{}", &password);

    get_folder_content(login, password);
}
