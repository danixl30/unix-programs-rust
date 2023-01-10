use std::process::Command;

fn execute_command(command: String, args: Vec<String>) -> String {
    let output = Command::new(command).args(args).output().unwrap();
    let value = String::from_utf8(output.stdout).unwrap();
    return value.clone();
}

fn main() {
    let data = execute_command(String::from("bash"), vec![String::from("-c"), String::from("ulimit -u")]);
    println!("Num max Pid: {}", data);
}
