use std::process::{Child, Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// A wrapper around a GDB process
#[derive(Debug)]
pub struct Gdb {
    process: Child,
    command_sender: Sender<String>,
    output_receiver: Receiver<String>,
}

impl Gdb {
    /// Spawn a new GDB process
    pub fn new(args: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut process = Command::new("gdb")
            .args(args)
            // .arg("--interpreter=mi3")  // Use machine interface for better parsing
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let (command_sender, command_receiver) = mpsc::channel();
        let (output_sender, output_receiver) = mpsc::channel();

        // Spawn a thread to handle command sending
        let stdin = process.stdin.take().ok_or("Failed to get stdin")?;
        let stdout = process.stdout.take().ok_or("Failed to get stdout")?;

        thread::spawn(move || {
            let mut stdin = stdin;
            while let Ok(command) = command_receiver.recv() {
                if let Err(e) = writeln!(stdin, "{}", command) {
                    eprintln!("Failed to send command to GDB: {}", e);
                    break;
                }
                if let Err(e) = stdin.flush() {
                    eprintln!("Failed to flush stdin: {}", e);
                    break;
                }
            }
        });

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if let Err(_) = output_sender.send(line) {
                            break; // Receiver was dropped
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from GDB stdout: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(Gdb {
            process,
            command_sender,
            output_receiver,
        })
    }

    /// Send a command to the GDB process
    pub fn send_command(&self, command: String) -> Result<(), Box<dyn std::error::Error>> {
        self.command_sender.send(command)?;
        Ok(())
    }

    /// Try to receive output from GDB (non-blocking)
    pub fn try_receive_output(&self) -> Option<String> {
        self.output_receiver.try_recv().ok()
    }

    /// Receive output from GDB (blocking)
    pub fn receive_output(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.output_receiver.recv()?)
    }

    /// Check if the GDB process is still running
    pub fn is_running(&mut self) -> bool {
        self.process.try_wait().map_or(true, |status| status.is_none())
    }

    /// Get the process ID of the GDB process
    pub fn pid(&self) -> u32 {
        self.process.id()
    }

    /// Send a signal to the GDB process
    pub fn send_signal(&mut self, signal: nix::sys::signal::Signal) -> Result<(), Box<dyn std::error::Error>> {
        let pid = self.process.id();
        nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), signal)?;
        Ok(())
    }

    /// Send SIGINT (interrupt) to the GDB process
    pub fn send_sigint(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_signal(nix::sys::signal::Signal::SIGINT)
    }

    /// Send SIGTERM to the GDB process
    pub fn send_sigterm(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_signal(nix::sys::signal::Signal::SIGTERM)
    }

    /// Send SIGKILL to the GDB process (force kill)
    pub fn send_sigkill(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_signal(nix::sys::signal::Signal::SIGKILL)
    }
}

impl Drop for Gdb {
    fn drop(&mut self) {
        let _ = self.send_command("quit".into());

        std::thread::sleep(std::time::Duration::from_millis(100));

        if self.is_running() {
            let _ = self.process.kill();
        }

        let _ = self.process.wait();
    }
}
