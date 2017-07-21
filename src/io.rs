use std::io::Write;
use std::str::FromStr;
use std::fs::File;
use std::fs::OpenOptions;

use imap::mailbox::Mailbox;
use bson::{encode_document, Document};

fn serialize(mailbox: &Mailbox) -> Document {
    let mut bdoc = Document::new();
    bdoc.insert("unseen", &mailbox.unseen.unwrap().to_string());
    bdoc.insert("exists", &mailbox.exists.to_string());
    bdoc.insert("recent", &mailbox.recent.to_string());
    bdoc
}

pub fn save(mailbox: &Mailbox) {
    let filename = "state.bson";
    let doc = serialize(mailbox);
    let mut buf = Vec::new();

    encode_document(&mut buf, &doc).unwrap();
    let mut file: File = OpenOptions::new().write(true).open(filename).unwrap_or(
        File::create(filename).expect(&format!("Can't create file {}", filename))
    );
    file.write(buf.as_slice()).unwrap();
}
