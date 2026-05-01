
use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
         if error.kind() == ErrorKind::NotFound {
           File::create("hello.txt").unwrap_or_else(|error| {
              panic!("Problema pra criar arquivo :{:?}", error);
           })
         } else {
              panic!("Problema pra abrir o arquivo: {:?}", error);
         }
    });
    println!("Arquivo {:?} criado ", greeting_file);
}
