fn main() {
    println!("Hello, world!");

    conteudo_opcional();
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("Conteudo: {}", valor),
        None => println!("Arquivo nao existe")  
    }

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo{
        println!("Agora sim tenho um valor: {}", valor);
    }
}

fn ler_arquivo(caminho: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
    //OR
    //None
}