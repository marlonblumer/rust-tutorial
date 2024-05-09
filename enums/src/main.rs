fn main() {
    println!("Fim de semana? {}",eh_fim_semana(DiaDaSemana::Domingo));

    cores();
}

#[allow (dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_semana(dia: DiaDaSemana) -> bool {
    match dia {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

#[allow (dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tupla
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8} //struct
}

fn cores() {
//    let cor = Color::RgbColor(0,0,0);
    let cor = Color::CymkColor{cyan: 88, magenta: 120, yellow: 50, black: 255};

    println!("Cor: {}", match cor {
        Color::Red => "Vermelho",
        Color::Green => "Verde",
        Color::Blue => "Azul",
        Color::RgbColor(0,0,0) | Color::CymkColor{cyan: _, magenta: _, yellow: _, black: 255} => "Preta",
        Color::RgbColor(_,_,_) => "Outra cor",
        Color::CymkColor{cyan: _, magenta: _, yellow: _, black: _} => "Cor CMYK desconhecido"
    });
}
