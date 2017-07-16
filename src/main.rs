extern crate imap;
extern crate openssl;
extern crate getopts;
extern crate iconv;

mod auth;
mod data_mining;

use std::num::ParseIntError;
use std::fs::{remove_file};
use std::env;

use getopts::Options;
use getopts::Matches;

use data_mining::{ImapClient, StatusItem};
use auth::Auth;

enum LibErr {
    AuthFail,
    ConnectFail,
    YetAnotherErr,
}

enum AppErr {
    Base(LibErr),
    // AuthFail(AuthFail),
    // ConnectFail(ConnectFail),
    ANother,
}

impl From<LibErr> for AppErr {
    fn from(err: LibErr) -> Self {
        AppErr::Base(err)
    }
}

fn throwable_wrapper(state: u8) -> Result<u8, AppErr> {
    let val = throwable_fn(state)? + 1;
    if val > 6 {
        Err(AppErr::ANother)
    } else {
        Ok(val)
    }
}

fn throwable_fn(state: u8) -> Result<u8, LibErr> {
    if state < 2 {
        Ok(1)
    } else if state == 1 {
        Err(LibErr::AuthFail)
    } else {
        Err(LibErr::ConnectFail)
    }
}

fn main() {
    let res = match throwable_wrapper(1) {
        Ok(x) => { x },
        Err(AppErr::Base(liberr)) => match liberr {
            LibErr::AuthFail => { panic!("liberr authfail") },
            _ => { panic!("qwe asd") },
            // LibErr::ConnectFail => { panic!("liberr connectfail") },
        },
        Err(AppErr::ANother) => { panic!("apperr ron") },
    };
    // println!("{}", res);

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

    let auth_obj = Auth::new(filename);
    
    let mut client = ImapClient::new(&auth_obj);
    client.login(&auth_obj);
    let folders = client.get_folders();
    for item in &folders {
        println!("{}", item.name);
    }

    let folder = &folders[5];
    let mailbox = client.select(&folder);
    println!("{} has {} messages", folder.name, mailbox);
    // TODO: how to notify about new messages.
    // 1. Select folder;
    // 2. Save a sequence number of unseen message;
    // 3. Get a status of the folder with argument UNSEEN;
    // 4. Save sum of unseen messages;
    // 5. Get new sequence number and new sum.
    // 6. Check that the sequence number is higher than saved sequence number. If new number is
    //    higher then it means you get new messages. How many new messgas you get is number of
    //    unseen messages from select command.
    // let folder_content = client.get_folder_content(&folder);
    // for item in folder_content {
        // println!("{}", item);
    // }

    let status = client.folder_status(folder, &vec![StatusItem::Messages]);
    println!("");
    for item in &status {
        println!("{}", item);
    }

    client.logout();
}
