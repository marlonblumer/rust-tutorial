fn main(){
  ownership();
}

fn ownership(){
  let var_string = String::from("Opa, agora vai.");
  rouba(var_string);

  //erro porque a var_string foi movida para dentro da função rouba. Essa é a forma que Rust trabalha.
  //println!("{}", var_string);
}

fn rouba(v_string: String){
  println!("{}", v_string);
}