extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

enum Command {
    SyncAndExit,
    SyncNow,
    ExitNow,
}

fn main() {
    let stdin = stdin();
    let mut std_out = stdout().into_raw_mode().unwrap();

    write!(std_out,
            "q to exit. Type stuff, use alt, and so on.\n\r{}",
            termion::cursor::Hide)
            .unwrap();
    std_out.flush().unwrap();

    let (send, recv) = mpsc::channel();

    let worker = thread::spawn(move || {
        let mut std_out = stdout().into_raw_mode().unwrap();
        let mut run = true;
        while run {
            run = match recv.recv_timeout(Duration::from_millis(1000)) {
                Ok(Command::SyncAndExit) => {
                    write!(std_out, "stop").unwrap();
                    std_out.flush().unwrap();
                    false
                },
                Ok(Command::SyncNow) => {
                    write!(std_out, "sync").unwrap();
                    std_out.flush().unwrap();
                    true
                },
                Ok(Command::ExitNow) => {
                    write!(std_out, "stop now").unwrap();
                    std_out.flush().unwrap();
                    break;
                },
                Err(mpsc::RecvTimeoutError::Timeout) => true,
                _ => true,
            };
            write!(std_out, "(work)").unwrap();
            std_out.flush().unwrap();
        }
    });



    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') | Key::Esc => {
                send.send(Command::SyncAndExit).unwrap();
                let _res = worker.join();
                break;
            },
            Key::Char('s') => {
                send.send(Command::SyncNow)
            },
            Key::Ctrl('c') => {
                send.send(Command::ExitNow).unwrap();
                let _res = worker.join();
                break;
            },
            _ => Ok(())
        }.unwrap();
        //write!(std_out, "\r\n").unwrap();
        //std_out.flush().unwrap();
    }
    write!(std_out, "\r\n").unwrap();
    write!(std_out, "{}", termion::cursor::Show).unwrap();
}
