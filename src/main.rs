extern crate imap;
extern crate openssl;
extern crate getopts;
extern crate iconv;
#[macro_use(bson, doc)]
extern crate bson;
extern crate serde;

mod data_mining;
mod io;
mod account;
mod auth;

use std::thread::{sleep, spawn, /*JoinHandle*/};
use std::cell::RefCell;
use io::save;
use std::sync::Arc;

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
    // let a = Arc::new(RefCell::new("qwe".to_owned()));
    // let q = a.clone();
    // (*q).borrow_mut().push('q');
    // println!("{}", (*a.borrow()));
    // let h = spawn(move || {
        // Arc::get_mut(&mut q).unwrap().push('q');
        // println!("{}", q);
    // });
    // h.join();
    // print!("{}", a);

    
    // let mut client = ImapClient::new(&auth);
    // client.login(&auth);
    // let folders = client.get_folders();
    // for item in &folders {
        // println!("{}", item.name);
    // }

    // let folder = &folders[5];
    // let mailbox = client.exam(&folder);
    // println!("{} has {} messages", folder.name, mailbox);
    // save(&mailbox);
    // let status = client.folder_status(folder, &vec![StatusItem::Messages, StatusItem::Recent, StatusItem::Unseen]);
    // println!("");
    // for item in &status {
        // print!("{}", item);
    // }
    // client.logout();
}
