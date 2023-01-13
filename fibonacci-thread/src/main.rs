use std::thread::{self, JoinHandle}; //Se importa lo necesario para trabajar con 'hilos'

use rand::Rng; //Se importa lo necesario para generar números aleatorios

//Se genera el fibonacci del argumento 'top'
fn fibonacci(top: i16) -> i64 {
    let mut result: i64 = 1;
    let mut prev = 0;
    for _ in 1..=top { //Se suman todos los números enteros sucesivamente de 1 hasta top, de acuerdo a la sucesión de fibonacci (resultado = n + (n-1) desde n=1 hasta n=top)
        let temp_2 = result;
        result += prev;
        prev = temp_2;
    }
    return result;
}

//Se obtiene un número entero ubicado en el intervalo [start,end]
fn get_random(start: i16, end: i16) -> i16 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(start..end);
}

//Cada hilo, individualmente, calculará 50000 fibonacci y los sumará entre si, utilizando esta funcion
fn roll() -> i64 {
    let mut sum = 0;
    for _ in 1..=50000 {
        let num = get_random(0, 19); //Se obtiene un numero aleatorio
        sum += fibonacci(num); //Se calcula su fibonacci y se agrega a la suma previa
    }
    sum //La funcion retorna el valor de la suma total de los 50000 fibonacci
}

fn main() {
    let mut handlers: Vec<JoinHandle<i64>> = vec![]; //Se crea un arreglo para guardar referencias a cada hilo
    for _ in 1..=20 {
        handlers.push(thread::spawn(|| roll())); //Se instancia cada uno de los 20 hilos, se les asigna que ejecuten la funcion roll() y se guarda una referencia a los mismos en el arreglo 'handlers'
    }
    let sum = handlers.into_iter() //Se utiliza un iterador sobre el arreglo
        .map(|handler| handler.join().unwrap()) //Se itera: A cada elemento de la coleccion se le aplica la funcion descrita y se genera una nueva coleccion con los resultados. join() espera a que el hilo finalice su ejecucion mientras que unwrap() extrae el resultado generado por la funcion que ejecutaba ese hilo
        .reduce(|acc, value| acc + value); //De la coleccion generada por map(), se suman entre si los valores de cada elemento, y se 'reducen' a una unica variable cuyo valor equivale al resultado de la suma
    println!("Result: {}", sum.unwrap()); //Se imprime el resultado almacenado en sum (suma de los 1000000 fibonacci)
}
