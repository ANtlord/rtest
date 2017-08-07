use std::hash::{Hash, Hasher};

use std::cmp::{PartialEq, Eq};
use imap::mailbox::Mailbox;
use data_mining::FolderData;

pub struct Folder {
    pub data: FolderData,
    pub mailbox: Mailbox,
    pub status: FolderStatus,
}

impl Folder {
    pub fn new(data: FolderData, mailbox: Mailbox) -> Self {
        Self{ mailbox: mailbox, data: data, status: FolderStatus::new() }
    }
    fn name<'a>(&'a self) -> &'a str {
        &self.data.name
    }
}

pub struct FolderStatus {
    messages: u64,
    recent: u64,
    unseen: u64,
}

impl FolderStatus {
    pub fn new() -> Self {
        Self { messages: 0, recent: 0, unseen: 0 }
    }
    pub fn update(&mut self, messages: u64, recent: u64, unseen: u64) {
        self.messages = messages;
        self.recent = recent;
        self.unseen = unseen;
    }
}

impl Hash for Folder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for FolderStatus {
    fn eq(&self, other: &Self) -> bool {
        self.messages == other.messages &&
        self.recent == other.recent &&
        self.unseen == other.unseen
    }
}
impl Eq for FolderStatus {}
