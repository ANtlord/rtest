pub mod folder;

use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;
use std::iter::Map;
use std::slice::Iter;
use imap::mailbox::Mailbox;

use auth::Auth;
use data_mining::{ImapClient, FolderData};
use self::folder::Folder;

pub struct App {
    auth: Auth,
    client: ImapClient,
    folders: HashSet<Folder>,
}

fn is_equal(mb1: &Mailbox, mb2: &Mailbox) -> bool {
    mb1.unseen == mb2.unseen && mb1.exists == mb2.exists && mb1.recent == mb2.recent
}

struct Message {
    sender: String,
    subject: String,
}

impl App {
    pub fn new(auth: Auth) -> Self {
        let mut client = ImapClient::new(&auth);
        client.login(&auth);
        let folders_data = client.get_folders();
        let mailboxes: Vec<Mailbox> = folders_data.iter().map(|x| client.exam(&x)).collect();
        let data_zip = mailboxes.into_iter().zip(folders_data.into_iter());
        let folders: HashSet<Folder> = data_zip.map(|(x, y)| Folder{ mailbox: x, data: y }).collect();
        Self { auth: auth, client: client, folders: folders }
    }

    /// returns list of new messages.
    pub fn check_new_messages(&mut self) -> Vec<Message> {
        // let folder = &self.folders.get("INBOX").unwrap();
        // let mailbox = self.client.exam(&folder);
        // is_equal()
        vec![Message{ sender: "test@example.org".to_string(), subject: "Top secret".to_string() }]
    }
}
