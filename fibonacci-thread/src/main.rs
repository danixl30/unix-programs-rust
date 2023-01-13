use std::thread::{self, JoinHandle};

use rand::Rng;

fn fibonacci(top: i16) -> i64 {
    let mut result: i64 = 1;
    let mut prev = 0;
    for _ in 1..=top {
        let temp_2 = result;
        result += prev;
        prev = temp_2;
    }
    return result;
}

fn get_random(start: i16, end: i16) -> i16 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(start..end);
}

fn roll() -> i64 {
    let mut sum = 0;
    for _ in 1..=50000 {
        let num = get_random(0, 19);
        sum += fibonacci(num);
    }
    sum
}

fn main() {
    let mut handlers: Vec<JoinHandle<i64>> = vec![];
    for _ in 1..=20 {
        handlers.push(thread::spawn(|| roll()));
    }
    let sum = handlers.into_iter()
        .map(|handler| handler.join().unwrap())
        .reduce(|acc, value| acc + value);
    println!("Result: {}", sum.unwrap());
}
