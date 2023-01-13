use nix::sys::wait::{wait, waitpid};
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::fork;
use nix::unistd::{alarm, pause};
use nix::sys::signal::*;
use std::io::prelude::*;

// Función para el manejo de SIGALRM
extern fn signal_handler(v: nix::libc::c_int) {
    println!{ "signal_handler {}", v };
}

// Función para ejecutar SIGALRM
fn set_alarm(sec: i32) {
    alarm::set(sec.try_into().unwrap());
}

fn main() {
    // Pipes de comunicación 
    let (mut reader_2, mut writer_2) = os_pipe::pipe().unwrap();
    let (mut reader, mut writer) = os_pipe::pipe().unwrap();
    unsafe { 
        // Registramos el handler de señal para este proceso
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
        Parent { child } => { // proceso padre
            drop(writer_2); // quitamos descriptores innecesarios
            drop(reader);
            let pid_2 = fork(); // crear subproceso 
            match pid_2.expect("Error creating child 2") {
                Parent { child } => { waitpid(child, None).unwrap(); } // no necesitamos al padre
                Child => {
                    let mut data: [u8;2] = [0;2]; // búfer
                    for _ in 1..=10 { // se repite 10 veces
                        set_alarm(2); // ejecutamos SIGALRM
                        pause();
                        writer.write("C2".as_bytes()).unwrap(); // escribimos cuando SIGALRM se lance
                        reader_2.read_exact(&mut data).unwrap(); // leemos
                        println!("Data from C1, {}", std::str::from_utf8(&data).unwrap()); // imprimimos
                    }
                }
            }
        }

        Child => { // proceso hijo
            drop(reader_2); // quitamos descriptores innecesarios
            drop(writer);
            let mut data: [u8;2] = [0;2]; // búfer
            for _ in 1..=10 {
                reader.read_exact(&mut data).unwrap(); // leemos
                writer_2.write(&data).unwrap(); // escribimos
            }
        }
    }
    wait();
}
