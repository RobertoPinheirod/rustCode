fn primeiras_palavras(s : &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item==b' ' {
            return i;
        }
    }
    s.len()
} 

fn main() {
    let mut s = String::from("Fala compadre");
    let palavra = primeiras_palavras(&s);
    print!("\n O espaco comeca em {} \n", palavra);
    s.clear();
}
