use std::{time::Instant, process::Command, env};


fn execute_command(command: String, args: Vec<String>) {
    let mut child = Command::new(command).args(args).spawn().unwrap();
    child.wait().unwrap();
}

fn time_prog(command: String, args: Vec<String>) -> u64 {
    let start = Instant::now();
    execute_command(command, args);
    return start.elapsed().as_secs();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not program");
    }
    let time = time_prog(
        args[1].clone(), 
        args.into_iter().enumerate().filter(|&(i, _)| i > 1).map(|(_, e)| e).collect()
    );
    println!("Time: {}", time);
}
