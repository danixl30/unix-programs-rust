use std::{time::Instant, process::Command, env}; //Librerías externas utilizadas

//Función que recibe un comando como string y sus argumentos, los pasa a la consola de comandos y lo ejecuta
fn execute_command(command: String, args: Vec<String>) {
    let mut child = Command::new(command).args(args).spawn().unwrap(); 
    child.wait().unwrap();
}

//Función que ejecuta el comando dado por parametro y determina el tiempo que tomó ejecutarlo
fn time_prog(command: String, args: Vec<String>) -> u64 {
    let start = Instant::now(); //Se marca el tiempo justo antes de ejecutar el comando
    execute_command(command, args);
    return start.elapsed().as_secs(); //Se retorna el tiempo que ha pasado desde la marca hecha anteriormente
}

fn main() {
    let args: Vec<String> = env::args().collect(); //Se toman los argumentos del comando de entre todas las variables del entorno
    if args.len() < 2 { //Asegurarse de que no hayan menos de 2 argumentos en el comando, en tal caso no se considera válido
        panic!("Not program");
    }
    let time = time_prog(
        args[1].clone(), //Se introduce el comando
        args.into_iter().enumerate().filter(|&(i, _)| i > 1).map(|(_, e)| e).collect() //Para los argumentos, se itera el vector y se filtran los caracteres innecesarios
    );
    println!("Time: {}", time); //Se imprime en pantalla el tiempo calculado por la función time_prog
}
