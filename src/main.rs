extern crate getopts;
extern crate imap;
extern crate openssl;
mod auth;
mod data_mining;

use std::fs::{remove_file};
use std::env;

use getopts::Options;

use auth::Auth;
use data_mining::get_folder_content;

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
    let auth_obj = Auth::new(filename);

    println!("{}", &auth_obj.login);

    get_folder_content(auth_obj.login, auth_obj.password);
}
