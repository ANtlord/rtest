use std::collections::HashMap;

use imap::mailbox::Mailbox;

use auth::Auth;
use data_mining::{ImapClient, StatusItem, FolderData};

pub struct App<'a> {
    auth: Auth,
    client: ImapClient,
    folder_list: Vec<Folder>,
    folders: HashMap<&'a str, &'a Folder>,
}

struct Folder {
    data: FolderData,
    mailbox: Mailbox
}

fn is_equal(mb1: &Mailbox, mb2: &Mailbox) -> bool {
    mb1.unseen == mb2.unseen && mb1.exists == mb2.exists && mb1.recent == mb2.recent
}

struct Message {
    sender: String,
    subject: String,
}

impl<'a> App<'a> {
    pub fn new(auth: Auth) -> Self {
        let mut client = ImapClient::new(&auth);
        client.login(&auth);

        // let mut folders_states: HashMap<&'a str, Folder> = ;
        // let a: &'a str = "qwe";
        let folders_data = client.get_folders();
        let mut app = Self { auth: auth, client: client, folder_list: vec![], folders: HashMap::new() };
        // let mailboxes = folders_data.iter().map(|x| app.client.exam(&x));
        for folder_data in folders_data {
            let folder_mailbox = app.client.exam(&folder_data);
            let folder = Folder{data: folder_data, mailbox: folder_mailbox};
            app.add(folder);
        }
        app
    }

    fn add<'b>(&'b mut self, folder: Folder) {
        self.folder_list.push(folder);
        let folder_ref = &self.folder_list[self.folder_list.len()];
        self.folders.insert(&folder_ref.data.name, &folder_ref);
    }

    /// returns list of new messages.
    pub fn check_new_messages(&mut self) -> Vec<Message> {
        // let folder = &self.folders.get("INBOX").unwrap();
        // let mailbox = self.client.exam(&folder);
        // is_equal()
        vec![Message{ sender: "test@example.org".to_string(), subject: "Top secret".to_string() }]
    }
}
