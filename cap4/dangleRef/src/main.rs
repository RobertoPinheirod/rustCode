fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String {
   let s = String::from("Salve");
   &s  // Tentando retornar uma referência para algo que vai sumir
}
