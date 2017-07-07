extern crate imap;
extern crate openssl;
extern crate getopts;
extern crate iconv;

mod auth;
mod data_mining;

use std::fs::{remove_file};
use std::env;

use getopts::Options;
use getopts::Matches;

use data_mining::{Action, ImapClient};
use auth::Auth;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("r", "erase", "erase auth file");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("a", "action", "possible actions: list", "ACTION");

    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        println!("this is help message");
        return;
    }

    let action = matches.opt_str("a").unwrap_or("none".to_owned());
    let _ = match action.as_str() {
        "list" => Action::List,
        _ => Action::None,
    };

    let filename = "auth.txt";
    if matches.opt_present("r") {
        remove_file(filename).unwrap_or(())
    }

    let auth_obj = Auth::new(filename);
    
    let mut client = ImapClient::new(&auth_obj);
    client.login(&auth_obj);
    let folders = client.get_folders();
    for item in &folders {
        println!("{}", item.name);
    }
    client.logout();
}
