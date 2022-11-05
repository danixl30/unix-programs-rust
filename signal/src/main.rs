use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::fork;
use std::io::prelude::*;
use std::time::Duration;

fn main() {
    let (mut reader, mut writer) = os_pipe::pipe().unwrap();
    let (mut reader_2, mut writer_2) = os_pipe::pipe().unwrap();
    let pid = fork();
    match pid.expect("Error during creating child") {
        Parent { child } => {
            drop(reader);
            drop(writer_2);
            let pid_2 = fork();
            match pid_2.expect("Error creating child 2") {
                Parent { child } => {}
                Child => {
                    let mut data: [u8;2] = [0;2];
                    for _ in 1..=10 {
                        writer.write("C1".as_bytes()).unwrap();
                        reader_2.read_exact(&mut data).unwrap();
                        println!("Data from C1, {}", std::str::from_utf8(&data).unwrap());
                        std::thread::sleep(Duration::from_millis(2000));
                    }
                }
            }
        }

        Child => {
            drop(writer);
            drop(reader_2);
            let mut data: [u8;2] = [0;2];
            for _ in 1..=10 {
                reader.read_exact(&mut data).unwrap();
                println!("Data from C2, {}", std::str::from_utf8(&data).unwrap());
                writer_2.write(&data).unwrap();
            }
        }
    }
    wait();
}
