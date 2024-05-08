fn main() {
    let notas: [f32; 4] = [10.0, 9.0, 6.5, 9.1];

    for nota in notas {
        println!("Nota: {}", nota);
    }

    for i in 0..notas.len() {
        println!("Nota-{}: {}", i+1, notas[i]);
    }

    matriz();
}

fn matriz() {
    let matriz: [[f32; 3]; 2]  = [
        [1.0, 2.2, 3.1],
        [0.4, 0.6, 1.1]
    ];

    for m1 in 0..matriz.len(){
        for m2 in 0..matriz[m1].len(){
            println!("M-[{}][{}]: {}", m1, m2, matriz[m1][m2]);
        }
    }
    //outra forma
    for linha in matriz {
        for item in linha {
            println!("Item: {}", item);
        }
    }
}
