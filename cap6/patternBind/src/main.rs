#[derive(Debug)] // pra poder inspecionar o estado
enum BrState {
    RioDeJaneiro,
    SaoPaulo,
    Minas,
}

enum Coin{
   Penny,
   Nickel,
   Dime,
   Quarter(BrState),
}


fn value_in_cents(coin:Coin) -> u8{
   match coin{
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(estado) => {
                                   println!(" Quarto do estado {:?} !" , estado);
                                   25
                               }
   }
}

use std::io;


fn main() {
    let  moeda : Coin;
    let mut numero_str = String::new();
    io::stdin().read_line(&mut numero_str).unwrap();
    let numero: i32 = numero_str.trim().parse().expect("Por favor, digite um número válido!");

    match numero {
          0..=1 => moeda = Coin::Penny,
          2..=5 => moeda = Coin::Nickel,
          6..=10 => moeda = Coin::Dime,
          11..=25 => moeda = Coin::Quarter(BrState::RioDeJaneiro),
          _      => {
            println!("Valor fora da faixa, atribuindo Penny por padrão.");
            moeda = Coin::Penny;
        }
    }


    let valor = value_in_cents( moeda );
    println!("A moeda eh : {:?}", valor);
}
