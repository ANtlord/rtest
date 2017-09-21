use std::hash::{Hash, Hasher};

use std::cmp::{PartialEq, Eq};
use imap::mailbox::Mailbox;
use data_mining::FolderData;

pub struct Folder {
    pub data: FolderData,
    pub mailbox: Mailbox,
    pub status: FolderStatus,
}

/// Provides folder metrics.
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

    fn update(&mut self, messages: u64, recent: u64, unseen: u64) {
        self.messages = messages;
        self.recent = recent;
        self.unseen = unseen;
    }

    pub fn update_from_status_command(&mut self, response: &Vec<String>) {
        println!("{:?}", response);
        let status_data: Vec<&str> = response[0].split(" ").collect();
        let mut vals = vec![];
        let len = status_data.len();
        for i in 3..len {
            if i % 2 == 1 {
                continue;
            }
            let val = str::replace(status_data[i].trim(), ")", "").parse::<u64>().unwrap();
            vals.push(val);
        }
        self.update(vals[0], vals[1], vals[2]);
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_from_status_command() {
        let mut fs = FolderStatus::new();
        assert_eq!(fs.messages, 0);
        assert_eq!(fs.recent, 0);
        assert_eq!(fs.unseen, 0);
        let status_command_response = vec![
            "* STATUS INBOX (MESSAGES 90 RECENT 34 UNSEEN 44)\r\n".to_owned(),
            "a11 OK STATUS Completed.\r\n".to_owned(),
        ];
        fs.update_from_status_command(&status_command_response);
        assert_eq!(fs.messages, 90);
        assert_eq!(fs.recent, 34);
        assert_eq!(fs.unseen, 44);
    }
}
