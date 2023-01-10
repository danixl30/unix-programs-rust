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

fn main() {
    let mut sum = 0;
    for _ in 1..=1000000 {
        let num = get_random(0, 19);
        sum += fibonacci(num);
    }
    println!("Result: {}", sum);
}
