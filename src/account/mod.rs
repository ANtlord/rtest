pub mod folder;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use imap::mailbox::Mailbox;

use auth::Auth;
use data_mining::{ImapClient, StatusItem};
use self::folder::Folder;

use std::thread::{sleep, spawn, /*JoinHandle*/};
use std::time::Duration;
use std::sync::mpsc::channel;

use std::cell::RefCell;

pub struct Account {
    inner: Arc<Mutex<AccountInner>>
}

struct AccountInner {
    auth: Auth,
    client: ImapClient,
    folders: HashMap<String, Folder>,
}

/// Compares number of all messages, number of recent messages, sequence number of the first unseen
/// message in mailbox.
fn is_equal(mb1: &Mailbox, mb2: &Mailbox) -> bool {
    mb1.unseen == mb2.unseen && mb1.exists == mb2.exists && mb1.recent == mb2.recent
}

struct Message {
    sender: String,
    subject: String,
}

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
impl Account {
    pub fn new(auth: Auth) -> Self {
        println!("Authorization...");
        Self { inner: Arc::new(Mutex::new(AccountInner::new(auth))) }
    }

    pub fn listen(&self) {
        println!("Looking for new emails...");
        let inner = self.inner.clone();
        let handle = spawn(move || {
            loop {
                inner.lock().unwrap().check_new_messages();
                sleep(Duration::from_secs(20));
            }
        });        
        handle.join();
    }
}
impl AccountInner {
    fn new(auth: Auth) -> Self {
        let mut client = ImapClient::new(&auth);
        client.login(&auth);
        let folders_data = client.get_folders();
        let mailbox_res: Vec<Option<Mailbox>> = folders_data.iter().map(|x| client.exam(&x)).collect();
        let mailboxes: Vec<Mailbox> = mailbox_res.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
        let data_zip = mailboxes.into_iter().zip(folders_data.into_iter());
        let folder_generator = data_zip.map(|(x, y)| (y.name.to_owned(), Folder::new(y, x)));
        let folders: HashMap<String, Folder> = folder_generator.collect();
        Self { auth: auth, client: client, folders: folders }
    }

    /// returns list of new messages.
    fn is_new_messages_exist(&mut self) -> bool {
        let folder = self.folders.get("INBOX").unwrap();
        let mailbox = self.client.exam(&folder.data);
        if is_equal(&mailbox.unwrap(), &folder.mailbox) {
            return false;
        }
        return true;
    }

    fn update_status(&mut self) {
        let mut folder = self.folders.get_mut("INBOX").unwrap();
        let status_items = vec![StatusItem::Messages, StatusItem::Recent, StatusItem::Unseen];
        let status = self.client.folder_status(&folder.data, &status_items);
        let vals: Vec<u64> = status.iter().map(parse_status_item_data).collect();
        folder.status.update(vals[0], vals[1], vals[2]);
    }

    pub fn check_new_messages(&mut self) -> Vec<Message> {
        let result = Vec::new();
        if !self.is_new_messages_exist() {
            return result;
        }
        self.update_status();
        return result;
    }
}

fn parse_status_item_data(data: &String) -> u64 {
    let status_item_data = data.split(" ").collect::<Vec<&str>>();
    let status_item_value = status_item_data[1].trim().parse::<u64>().unwrap();
    status_item_value
}
