fn main() {
    /*let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < 5 {
        println!("O valor é: {}", a[indice]);

        indice = indice + 1;
    }*/

    /*let a = [10, 20, 30, 40, 50];

    for elemento in a.iter() {
        println!("O valor é: {}", elemento);
    }*/

    /*for numero in (1..4).rev() {
        println!("{}!", numero);
    }
    println!("LIFTOFF!!!");*/

    let result: f64 = fah_celsius(15.0);

    println!("resultado fah_celsius: {}", result);

    let result: u32 = fib(14);

    println!("resultado fib: {}", result);
}

fn fah_celsius(temp: f64) -> f64{
    // converte temperatura de fahrenheit para celsius

    return (temp - 32.0) * (5.0 / 9.0);
}

fn fib(numero: u32) -> u32{
    // sem usar memoização
    if numero == 1 {
        return 1;
    } else if numero == 2 {
        return 1;
    } else {
        return fib(numero - 2) + fib(numero - 1);
    }
}
