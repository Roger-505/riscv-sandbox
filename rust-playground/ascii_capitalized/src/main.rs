fn main() {
    let mut v: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];

    ascii_capitalize(&mut v);
    println!("v: {}", v[0]);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        // Este es un ejemplo en el Rust book. Con respecto a analizar
        // permisos, el siguiente es totalmente valido ya que el compilador 
        // analiza los permisos de derecha a izquierda. Por tanto, la variable
        // up es redundante
        // v[0] = c.to_ascii_uppercase();
        let up = c.to_ascii_uppercase();
        v[0] = up;  // Rust va poner la referencia automaticamente
                    // Entonces, es exactamente lo mismo a (*v)[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}
