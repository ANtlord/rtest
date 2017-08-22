extern crate imap;
extern crate openssl;
extern crate getopts;
extern crate iconv;
#[macro_use(bson, doc)]
extern crate bson;
extern crate serde;
extern crate chrono;

mod data_mining;
mod io;
mod account;
mod auth;

use std::fs::{remove_file};
use std::env;

use getopts::Options;
use getopts::Matches;

use data_mining::{ImapClient, StatusItem};
use auth::Auth;
use account::Account;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("r", "erase", "erase auth file");
    opts.optflag("h", "help", "print this help menu");

    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        println!("this is help message");
        return;
    }

    let filename = "auth.txt";
    if matches.opt_present("r") {
        remove_file(filename).unwrap_or(())
    }

    let auth = Auth::new(filename);

    let acc = Account::new(auth);
    acc.listen();
}
