use imap::error::Error;
use imap::client::Client;
use imap::mailbox::Mailbox;
use std::net::TcpStream;
use openssl::ssl::{SslConnectorBuilder, SslMethod, SslStream};
use auth::Auth;
use iconv::Iconv;
use std::ops::Deref;

/// Wraps imap client and allows using it in several threads. Wraps raw responses into
/// abstractions.
pub struct ImapClient {
    imap_socket: Client<SslStream<TcpStream>>,
}

impl ImapClient {
    pub fn new(options: &Auth) -> Self {
        let imap_socket = Client::secure_connect(
            (options.host.as_str(), options.port), options.host.as_str(),
            SslConnectorBuilder::new(SslMethod::tls()).unwrap().build()
        ).unwrap();
        Self { imap_socket: imap_socket }
    }

    pub fn login(&mut self, options: &Auth) {
        self.imap_socket.login(&options.user, &options.pass).expect("cannot login");
    }

    /// Returns list of folders.
    pub fn get_folders(&mut self) -> Vec<FolderData> {
        let folder_data = self.imap_socket.list("\"\"", "*").expect("cannot get folders");
        let folders = folder_data.into_iter().map(|x| FolderData::new(&x)).collect();
        folders
    }

    /// Performs EXAMINE command applied to pointed folder. Sets pointed folder as current.
    /// Returns folder represenation as mailbox. Mailbox cab be None if there is no mailbox for
    /// pointed folder.
    pub fn exam(&mut self, folder: &FolderData) -> Option<Mailbox> {
        match self.imap_socket.examine(&folder.raw_name) {
            Ok(mb) => { return Some(mb) },
            Err(e) => match e {
                Error::NoResponse(str_vec) => {
                    println!("{}", str_vec.join(" "));
                    return None;
                }
                _ => { panic!("unknown error."); }
            }
        };
    }

    pub fn select(&mut self, folder: &FolderData) -> Option<Mailbox> {
        let mailbox = self.imap_socket.select(&folder.raw_name).expect("cannot select folder");
        Some(mailbox)
    }

    pub fn folder_status(&mut self, folder: &FolderData, items: &Vec<StatusItem>) -> Vec<String> {
        let arguments: Vec<String> = items.iter().map(|x| x.to_string()).collect();
        let item_str = format!("({})", arguments.join(" "));
        let status = self.imap_socket.status(&folder.raw_name, &item_str).expect("cannot get status");
        status
    }

    /// Returns number set of new messages in current folder.
    pub fn search_new_messages(&mut self) -> Vec<String> {
        let command = format!("UID SEARCH NEW");
        let response = self.imap_socket.run_command_and_read_response(&command).unwrap();
        let numbers: Vec<&str> = response[0].trim().split(" ").collect();
        numbers[ 2 .. ].iter().map(|x| x.to_string()).collect()
    }

    pub fn get_folder_content(&mut self, uid_set: &str, flag: FetchFlags) -> Vec<String> {
        match self.imap_socket.uid_fetch(uid_set, &flag.to_string()) {
            Ok(res) => res,
            Err(e) => { panic!("{}", e) }
        }
    }

    pub fn logout(&mut self) {
        self.imap_socket.logout().unwrap();
    }
}

pub enum FetchFlags {
    Body,
}

impl FetchFlags {
    fn to_string(&self) -> String {
        use self::FetchFlags::*;
        use std::env;
        match self {
            &Body => format!("BODY[{}]", env::var("BODY").unwrap()),
        }
    }
}

#[allow(dead_code)]
pub enum StatusItem {
    Messages,
    Recent,
    Uidnext,
    Uidvalidity,
    Unseen,
}

impl StatusItem {
    fn to_string(&self) -> String {
        use self::StatusItem::*;
        match self {
            &Messages => "MESSAGES".to_owned(),
            &Recent => "RECENT".to_owned(),
            &Uidnext => "UIDNEXT".to_owned(),
            &Uidvalidity => "UIDVALIDITY".to_owned(),
            &Unseen => "UNSEEN".to_owned(),
        }
    }
}

pub struct FolderData {
    pub name: String,
    raw_name: String,
    other: String,
}

impl FolderData {
    pub fn new(raw_string: &str) -> Self {
        let mut iconv = Iconv::new("UTF-8", "UTF-7").unwrap();

        let folder_from = raw_string.rfind(' ').unwrap();
        let raw_name = String::from(raw_string[folder_from .. raw_string.len()].trim());
        let folder: String = raw_name.as_str().replace("&", "+").replace(",", "/");
        
        let mut buf = Vec::new();
        iconv.convert(folder.trim().as_bytes(), &mut buf, 0).unwrap();
        Self {
            name: String::from_utf8(buf).unwrap(),
            raw_name: raw_name,
            other: (&raw_string[0 .. folder_from]).to_owned()
        }
    }
}
