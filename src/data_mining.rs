use imap::client::Client;
use std::net::TcpStream;
use openssl::ssl::{SslConnectorBuilder, SslMethod, SslStream};
use auth::Auth;
use iconv::Iconv;

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

    pub fn get_folders(&mut self) -> Vec<Folder> {
        let folder_data = self.imap_socket.list("\"\"", "*").expect("cannot get folders");
        let folders = folder_data.into_iter().map(|x| Folder::new(&x)).collect();
        folders
    }

    pub fn get_folder_content(&mut self, folder: &Folder) -> Vec<String> {
        self.imap_socket.list(&folder.raw_name, "*").expect("cannot get folder")
    }

    pub fn logout(&mut self) {
        self.imap_socket.logout().unwrap();
    }
}

pub struct Folder {
    pub name: String,
    raw_name: String,
    other: String,
}

impl Folder {
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

pub enum Action {
    List,
    None
}

