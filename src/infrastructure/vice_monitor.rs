use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ViceMonitor {
    command_tx: Option<Sender<String>>,
    disconnect_rx: Arc<Mutex<Receiver<()>>>,
}

impl ViceMonitor {
    pub fn new(address: String) -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        let (disconnect_tx, disconnect_rx) = mpsc::channel();

        thread::spawn(move || {
            Self::monitor_thread(address, command_rx, disconnect_tx);
        });

        Self { command_tx: Some(command_tx), disconnect_rx: Arc::new(Mutex::new(disconnect_rx)) }
    }

    pub fn check_disconnected(&self) -> bool {
        self.disconnect_rx.lock().is_ok_and(|rx| rx.try_recv().is_ok())
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
    fn monitor_thread(address: String, command_rx: Receiver<String>, disconnect_tx: Sender<()>) {
        thread::sleep(Duration::from_millis(2000));

        let Ok(mut stream) = TcpStream::connect(&address) else {
            let _ = disconnect_tx.send(());
            return;
        };

        let read_stream = stream.try_clone().expect("Failed to clone stream");
        let reader = BufReader::new(read_stream);
        let mut lines = reader.lines();

        stream.set_read_timeout(Some(Duration::from_millis(10))).ok();

        loop {
            if let Ok(command) = command_rx.try_recv() {
                let cmd = format!("{command}\n");
                if stream.write_all(cmd.as_bytes()).is_err() {
                    let _ = disconnect_tx.send(());
                    break;
                }
                if stream.flush().is_err() {
                    let _ = disconnect_tx.send(());
                    break;
                }
            }

            if let Some(result) = lines.next()
                && result.is_err() {
                    let _ = disconnect_tx.send(());
                    break;
                }

            thread::sleep(Duration::from_millis(10));
        }
    }
}
