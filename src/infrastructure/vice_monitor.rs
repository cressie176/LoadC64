use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct ViceMonitor {
    command_tx: Option<Sender<String>>,
}

impl ViceMonitor {
    pub fn new(address: String) -> Self {
        let (command_tx, command_rx) = mpsc::channel();

        thread::spawn(move || {
            Self::monitor_thread(address, command_rx);
        });

        Self { command_tx: Some(command_tx) }
    }

    pub fn send_quit(&self) {
        self.send_command("quit");
    }

    fn send_command(&self, command: &str) {
        if let Some(tx) = &self.command_tx {
            let _ = tx.send(command.to_string());
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn monitor_thread(address: String, command_rx: Receiver<String>) {
        thread::sleep(Duration::from_millis(2000));

        let Ok(mut stream) = TcpStream::connect(&address) else {
            return;
        };

        loop {
            let Ok(command) = command_rx.try_recv() else {
                thread::sleep(Duration::from_millis(10));
                continue;
            };

            let cmd = format!("{command}\n");
            if stream.write_all(cmd.as_bytes()).is_err() {
                break;
            }
            if stream.flush().is_err() {
                break;
            }
        }
    }
}
