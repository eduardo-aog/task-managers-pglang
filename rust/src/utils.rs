use std::io;

pub fn input(text: &str) -> String {
    println!("{}",text);
    let mut input_string: String = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Error al leer la linea");
    let trimmed: String = input_string.trim().to_string();
    return trimmed;
}