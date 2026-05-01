use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
    .expect("Nao achei hello.txt");
}
