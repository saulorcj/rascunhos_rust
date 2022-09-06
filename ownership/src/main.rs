fn main() {
    /*
    let mut s = String::from("olá");

    s.push_str(", mundo!");

    println!("{}", s);
    */

    /*
    let x = 5;
    let y = x;
    */

    /*
    let s1 = String::from("texto");
    let s2 = s1;

    println!("{}", s1);
    */

    /*
    let s1 = String::from("texto");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    */

    /*
    s1 = entrega_valor();

    let s2 = String::from("texto");

    let s3 = pega_e_entrega_valor(s2);
    */

    let s1 = String::from("texto");

    let (s2, tamanho) = calcula_tamanho(s1);

    println!("O tamanho de '{}' é {}", s2, tamanho);

}

/*
fn entrega_valor() -> String {
    let uma_string = String::from("olá");
    uma_string
}

fn pega_e_entrega_valor(uma_string: String) -> String {
    uma_string
}
*/

fn calcula_tamanho(s: String) -> (String, usize) {
    let tamanho = s.len();

    (s, tamanho)
}