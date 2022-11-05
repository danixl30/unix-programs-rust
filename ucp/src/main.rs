use std::{fs::{self, File}, env};

fn copy_file(origin: String, destinate: String, size: u64) {
    if size < 1 || size > 16384 {
        panic!("Size not supported");
    }
    let file = File::open(origin.clone()).unwrap();
    if size != file.metadata().unwrap().len() {
        panic!("Size not equals");
    }
    fs::copy(origin, destinate).unwrap();
    println!("File copied");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Not params");
    }
    let size = args[1].to_string().parse::<u64>().unwrap();
    copy_file(args[2].clone(), args[3].clone(), size);
}
