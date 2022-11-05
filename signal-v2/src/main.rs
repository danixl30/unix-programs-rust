use nix::sys::wait::{wait, waitpid};
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::fork;
use nix::unistd::{alarm, pause};
use nix::sys::signal::*;
use std::io::prelude::*;

extern fn signal_handler(v: nix::libc::c_int) {
    println!{ "signal_handler {}", v };
}

fn set_alarm(sec: i32) {
    alarm::set(sec.try_into().unwrap());
}

fn main() {
    let (mut reader_2, mut writer_2) = os_pipe::pipe().unwrap();
    let (mut reader, mut writer) = os_pipe::pipe().unwrap();
    unsafe { 
        sigaction(
            Signal::SIGALRM, 
            &SigAction::new(
                SigHandler::Handler(signal_handler), 
                SaFlags::empty(), 
                SigSet::empty())
            ).unwrap(); 
    }
    let pid = fork();
    match pid.expect("Error during creating child") {
        Parent { child } => {
            drop(writer_2);
            drop(reader);
            let pid_2 = fork();
            match pid_2.expect("Error creating child 2") {
                Parent { child } => { waitpid(child, None).unwrap(); }
                Child => {
                    let mut data: [u8;2] = [0;2];
                    for _ in 1..=10 {
                        set_alarm(2);
                        pause();
                        writer.write("C2".as_bytes()).unwrap();
                        reader_2.read_exact(&mut data).unwrap();
                        println!("Data from C1, {}", std::str::from_utf8(&data).unwrap());
                    }
                }
            }
        }

        Child => {
            drop(reader_2);
            drop(writer);
            let mut data: [u8;2] = [0;2];
            for _ in 1..=10 {
                reader.read_exact(&mut data).unwrap();
                writer_2.write(&data).unwrap();
            }
        }
    }
    wait();
}
