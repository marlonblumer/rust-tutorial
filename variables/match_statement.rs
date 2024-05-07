fn main(){
  let language = "PHP";

  

  let propose = match language{
    "PHP" => "Web dev",
    "Python" => "Data science",
    "Rust" => "Fast and security",
    _ => "Uso geral"
  };

  println!("Language: {} and propose: {}", language, propose);

}

