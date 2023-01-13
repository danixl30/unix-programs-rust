use rand::Rng;

//Calcular Fibonacci de un número
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

//Obtener un número aleatorio en el rango de start hasta end
fn get_random(start: i16, end: i16) -> i16 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(start..end);
}

fn main() {

    let mut sum = 0;

    //Itera 1 millón de veces
    for _ in 1..=1000000 {

        //"num" adquiere un valor aleatorio del 0 al 19
        let num = get_random(0, 19);
        //El Fibonacci de num se le suma a "sum"
        sum += fibonacci(num);
    }

    //Imprime por pantalla el resultado final
    println!("Result: {}", sum);
}
