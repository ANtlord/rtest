use std::hash::{Hash, Hasher};

use std::cmp::{PartialEq, Eq};
use imap::mailbox::Mailbox;
use data_mining::FolderData;


pub struct Folder {
    pub data: FolderData,
    pub mailbox: Mailbox,
}

impl Folder {
    fn name<'a>(&'a self) -> &'a str {
        &self.data.name
    }
}

impl Hash for Folder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for Folder {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl Eq for Folder {}
