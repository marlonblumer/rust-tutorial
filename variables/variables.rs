const PI:f32 = 3.14;
static mut VAR_GLOBAL:u8 = 1;

fn shad(){
  let a:i32 = 100;

  {
    let a:i32 = 200;
    println!("A dentro: {}", a)
  }

  println!("A fora: {}", a);
}

fn sum(val_1:u8, val_2:u8) -> u8{
  val_1 + val_2
}

fn main(){
  println!("PI value: {}", PI);
  
  unsafe{
    println!("Global: {}", VAR_GLOBAL);
  }

  let var1:i32 = 128;

  println!("Variable: {}, tamanho: {}", var1, std::mem::size_of_val(&var1));

  let decimal:f32 = 2.5;
  println!("Valor: {}", decimal);

  let var_bool:bool = true;
  //var_bool = true;
  //let mut var_bool:bool = false;
  println!("Bool value: {}, Bool space: {}", var_bool, std::mem::size_of_val(&var_bool));

  let var_char:char = 'M';
  println!("Valor do char: {}", var_char);

  shad();

  println!("Soma: {}",sum(1,4));
}