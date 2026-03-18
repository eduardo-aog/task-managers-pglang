mod utils;

fn main() {
    let mut cond: bool = true;
    println!("----- RusTask -----\nBienvenido");

    while cond {               // Esta función input no es nativa. Revisar utils.rs
        let read: String = utils::input("¿Qué operación desea realizar?");
        
        if read == "exit" {
            cond = false;
        }
    } 
    println!("Fin del programa.");
}