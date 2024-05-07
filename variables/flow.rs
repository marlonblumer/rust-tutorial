fn main(){
  let age:i8 = 17;
  let permission:bool = true;

  let condicao = if age >= 18 || age >= 16 && permission  { "Pode entrar" } else { "Não pode entrar" };
  println!("{}",condicao);

  repeticoes();
  fortest();
}

fn fortest(){
  for i in 1..11 {
    println!("For: {}", i);
  }
}


fn repeticoes(){
  let multiplicador:i8 = 5;
  let mut contador:i8 = 0;

  while contador < 10 {
    contador += 1;  
    println!("{} * {} = {}", contador, multiplicador, contador*multiplicador);
  }

  contador = 0;
  loop {
    contador += 1;
    if contador > 10 {
      break;
    }
    if contador == 5 {
      println!("Chegou na metade, pular o 5.");
      continue;
    }
    println!("Contador no loop: {}", contador);
  }

}