use std::thread::{self, JoinHandle};

use rand::Rng;

fn fibonacci(top: i16) -> i64 {
    let mut result: i64 = 1;
    let mut prev = 0;
    for _ in 1..=top {
        let temp_2 = result;
        result += prev + result;
        prev = temp_2;
    }
    return result;
}

fn get_random(start: i16, end: i16) -> i16 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(start..end);
}

fn roll() {
    for _ in 1..=50000 {
        let num = get_random(0, 19);
        let result = fibonacci(num);
        println!("{}", result);
    }
}

fn main() {
    let mut handlers: Vec<JoinHandle<()>> = vec![];
    for _ in 1..20 {
        let handler = thread::spawn(|| {
            roll();
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
}
