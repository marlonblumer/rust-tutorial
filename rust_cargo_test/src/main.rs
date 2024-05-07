fn main() {
    println!("Hello, world!");
    pattern_matching();
    errors_2();
    test_error();
}

fn pattern_matching() {
    for x in 1..20 {
        println!("{}: {}", x, match x {
            1 => "Só um?",
            2 | 3 => "Ficando melhor",
            4..=10 => "Quase lá.",
            _ if x % 2 == 0 => "Multiplo de 2.",
            _ => "Máximo" 
        });
    }
}

fn test_error() {
    let v = vec![1,2,3];
    println!("Vector: {}", v[2]);

    panic!("Erro proposital:");
    println!("Qual o valor: {}", v[11]);
}

fn errors_2() {
    println!("Mostrar o outro erro:");
    match result() {
        Ok(s) => println!("{}", s),
        Err(numero) => println!("{}", numero)
    }
}

fn result() -> Result<String, u8> {
    //Ok(String::from("Tudo ok."))
    Err(42)
}