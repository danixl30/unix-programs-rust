use std::process::Command;

//Función que ejecuta el comando, junto con sus argumentos y duvuelve el resultado de la ejecución 
fn execute_command(command: String, args: Vec<String>) -> String {
    let output = Command::new(command).args(args).output().unwrap();
    let value = String::from_utf8(output.stdout).unwrap();
    return value.clone();
}

fn main() {
//Se almacena en "data" el mayor número de procesos activos que puede tener un usuario a la vez
    let data = execute_command(String::from("bash"), vec![String::from("-c"), String::from("ulimit -u")]);
//Se imprime "data"   
    println!("Num max Pid: {}", data);
}
