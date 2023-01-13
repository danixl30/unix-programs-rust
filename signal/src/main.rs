use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::fork;
use std::io::prelude::*;
use std::time::Duration;

fn main() {
    // Crear pipes para la comunicación
    let (mut reader, mut writer) = os_pipe::pipe().unwrap();
    let (mut reader_2, mut writer_2) = os_pipe::pipe().unwrap();
    // Creamos el subproceso
    let pid = fork();
    match pid.expect("Error during creating child") {
        // proceso padre
        Parent { child } => {
            // Eliminamos los descriptores que no necesitamos en este proceso
            drop(reader);
            drop(writer_2);
            // Creamos otro subproceso
            let pid_2 = fork();
            match pid_2.expect("Error creating child 2") {
                // No necesitamos al padre
                Parent { child } => {}
                // Tercer subproceso
                Child => {
                    // Búfer de lectura
                    let mut data: [u8;2] = [0;2];
                    // Se repite 10 veces
                    for _ in 1..=10 {
                        // Escribimos el mensaje
                        writer.write("C1".as_bytes()).unwrap();
                        // Leemos la respuesta
                        reader_2.read_exact(&mut data).unwrap();
                        // Imprimimos la respuesta
                        println!("Data from C1, {}", std::str::from_utf8(&data).unwrap());
                        // Esperar 2 segundos
                        std::thread::sleep(Duration::from_millis(2000));
                    }
                }
            }
        }
        // Proceso hijo
        Child => {
            // No necesitamos estos descriptores
            drop(writer);
            drop(reader_2);
            // Búfer
            let mut data: [u8;2] = [0;2];
            // Se repite 10 veces
            for _ in 1..=10 {
                // Leemos
                reader.read_exact(&mut data).unwrap();
                // Imprimimos
                println!("Data from C2, {}", std::str::from_utf8(&data).unwrap());
                // Escribimos
                writer_2.write(&data).unwrap();
            }
        }
    }
    wait();
}
