use std::thread;
use std::sync::mpsc::TryRecvError;
use std::io::{stdout, Write};

use std::sync::mpsc;
use std::time::Duration;

use imap::client::Client;
use openssl::ssl::{SslConnectorBuilder, SslMethod};

pub fn get_folder_content(login: String, password: String) {
    let (tx, rx) = mpsc::channel();
    indicate_loading(rx);

    let h = thread::spawn(move || {
        let host = "imap.gmail.com";
        let mut imap_socket = Client::secure_connect(
            (host, 993), host,
            SslConnectorBuilder::new(SslMethod::tls()).unwrap().build()
        ).unwrap();

        let folders = imap_socket.list("\"\"", "*").expect("cannot get folders");

        imap_socket.login(&login, &password).expect("cannot login");
        let mb = imap_socket.select("INBOX").unwrap();

        imap_socket.logout().unwrap();
        tx.send(true).unwrap();
        thread::sleep(Duration::from_millis(500));
        println!("\n{}", mb);
        println!("Folders:");
        for item in folders {
            println!("{}", item);
        }
    });
    h.join().unwrap();
}

fn draw_loading(mut count: u8) -> u8 {
    thread::sleep(Duration::from_millis(500));
    if count % 3 == 0 {
        print!("\r   \r");
        count = 0;
    }
    print!(".");
    stdout().flush().unwrap();
    count + 1
}

fn indicate_loading(rx: mpsc::Receiver<bool>) {
    thread::spawn(move || {
        let mut count = 0;
        let mut is_done = false;
        while !is_done {
            count = draw_loading(count);
            match rx.try_recv() {
                 Ok(t)  => { is_done = t },
                 Err(TryRecvError::Disconnected) => { is_done = true },
                 Err(TryRecvError::Empty) => {}
            }
        }
    });
}
