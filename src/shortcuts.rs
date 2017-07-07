use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::io::{stdout, Write};
use std::time::Duration;
// pub fn get_folders(options: Auth) {
    // let (tx, rx) = mpsc::channel();
    // indicate_loading(rx);

    // let h = thread::spawn(move || {
        // let mut imap_socket = Client::secure_connect(
            // (options.host.as_str(), options.port), options.host.as_str(),
            // SslConnectorBuilder::new(SslMethod::tls()).unwrap().build()
        // ).unwrap();
        // imap_socket.login(&options.user, &options.pass).expect("cannot login");

        // let folders = imap_socket.list("\"\"", "*").expect("cannot get folders");
        // let mb = imap_socket.select("INBOX").unwrap();

        // imap_socket.logout().unwrap();
        // tx.send(true).unwrap();
        // thread::sleep(Duration::from_millis(500));
        // println!("\n{}", mb);
        // println!("Folders:");
        // for item in folders {
            // print!("{}", item);
        // }
    // });
    // h.join().unwrap();
// }

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
