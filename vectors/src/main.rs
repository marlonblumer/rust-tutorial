fn main() {
    
    vectors();
}

fn vectors() {
    //let mut notas:Vec<f32> = Vec::new();
    let mut notas:Vec<f32> = vec![1.1,2.2,3.3];

    //notas.push(8.1);
    //notas.push(2.2);
    println!("Capacidade: {}", notas.capacity());
    notas.push(6.5);
    println!("Capacidade depois de adicionar outro valor: {}", notas.capacity());

    for i in &notas {
        println!("Nota: {}", i);
    }

    println!("{:?}", notas);

    println!("Nota 8 = {}", match notas.get(7) {
        Some(n) => *n,
        None => 0.0
    });

    // if let Some(notas) = notas.pop() {
    //     println!("Ultimo valor: {}", notas);
    // }

    while let Some(notas) = notas.pop() {
        println!("Removendo: {}", notas);
    }
    println!("{:?}", notas);
}
