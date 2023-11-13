/*fn factorial(n: u64) -> u64 {
    if n ==0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let n = 5;
    let resultado = factorial(n);
    println!("El factorial de {} es : {}", n, resultado);
} */

/*use std::io;

fn es_palindromo(s: &str) -> bool {
    let s = s.trim().to_lowercase(); 
    if s.len() <= 1 {
        
        return true;
    } else if s.chars().next() == s.chars().last() {
        
        return es_palindromo(&s[1..s.len() - 1]);
    } else {
        return false;
    }
}

fn main() {
    println!("Ingrese una palabra para verificar si es un palíndromo:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    
    if es_palindromo(&input) {
        println!("La palabra es un palíndromo.");
    } else {
        println!("La palabra no es un palíndromo.");
    }
}*/

use std::io;

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    println!("Ingrese la longitud de la sucesión de Fibonacci:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    let length: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, ingrese un número válido.");
            return;
        }
    };

    println!("Sucesión de Fibonacci hasta el término {}:", length);
    for i in 0..length {
        print!("{} ", fibonacci(i));
    }
    println!();
}

