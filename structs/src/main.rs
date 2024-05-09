use core::f64;

fn main() {
    conta_corrente();
}

struct Conta {
    titular: Titular,
    saldo: f64
}

struct Titular {
    nome: String,
    sobrenome: String
}

impl Conta {
    fn sacar(&mut self, valor: f64){
        self.saldo -= valor;
    }
}

fn conta_corrente() {
    let mut conta: Conta = Conta {
        titular: Titular{
            nome: String::from("Marlon"),
            sobrenome: String::from("Sobrenome teste")
        },
        saldo: 880.50
    };

    conta.sacar(50.0);

    println!("Nome: {} {}, Saldo: {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);
}