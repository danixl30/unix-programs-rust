use std::{fs::{self, File}, env};

fn copy_file(origin: String, destinate: String, size: u64) {
    // El tamaño del archivo no puede ser menor a 1 ni mayor a 16KB
    if size < 1 || size > 16384 {
        panic!("Size not supported");
    }
    // Abrimos el archivo
    let file = File::open(origin.clone()).unwrap();
    // Validar tamaño
    if size != file.metadata().unwrap().len() {
        panic!("Size not equals");
    }
    // Copiar archivo
    fs::copy(origin, destinate).unwrap();
    println!("File copied");
}

fn main() {
    // Obtener argumentos del comando
    let args: Vec<String> = env::args().collect();
    // Validar argumentos
    if args.len() < 4 {
        panic!("Not params");
    }
    // Extraer tamaño del archivo
    let size = args[1].to_string().parse::<u64>().unwrap();
    // Ejecutar la funcion que copia el archivo
    copy_file(args[2].clone(), args[3].clone(), size);
}
