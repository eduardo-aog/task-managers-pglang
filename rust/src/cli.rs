use crate::task_manager::{Status, TaskManager};
use std::io::{self, Write};

const FILE_PATH: &str = "tasks.json";

pub fn run() {
    let mut manager = TaskManager::load_from_file(FILE_PATH); // Cargado del JSON

    println!("========================================");
    println!("          RusTask Manager");
    println!("  Escribe 'help' para ver los comandos");
    println!("========================================");

    loop {
        // Para mantener el programa en ejecución
        print!("task-cli> ");
        io::stdout().flush().unwrap(); // Asegura que el prompt se imprima antes de leer

        let mut input = String::new();

        // Leemos la entrada del usuario desde la consola
        if io::stdin().read_line(&mut input).is_err() {
            println!("error al leer la entrada");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Separamos la entrada por espacios
        let args: Vec<&str> = input.split_whitespace().collect();
        let command = args[0];

        match command {
            "add" => {
                if args.len() < 2 {
                    println!("error: descripción de la tarea necesaria");
                } else {
                    let description = args[1..].join(" ");
                    manager.add(description);
                    manager.save_to_file(FILE_PATH);
                }
            }
            "update" => {
                if args.len() < 3 {
                    println!(
                        "error: uso incorrecto\n quizas quisiste decir: update <id> <nueva descripción>"
                    );
                } else if let Ok(id) = args[1].parse::<i32>() {
                    let new_description = args[2..].join(" ");
                    manager.update_description(id, new_description);
                    manager.save_to_file(FILE_PATH);
                } else {
                    println!("error: el ID debe ser un número entero");
                }
            }
            "delete" => {
                if args.len() < 2 {
                    println!("error: ID de la tarea a eliminar necesario");
                } else if let Ok(id) = args[1].parse::<i32>() {
                    manager.delete(id);
                    manager.save_to_file(FILE_PATH);
                } else {
                    println!("error: el ID debe ser un número entero");
                }
            }
            "mark-in-progress" => {
                update_status_cli(&mut manager, &args, Status::InProgress);
            }
            "mark-done" => {
                update_status_cli(&mut manager, &args, Status::Done);
            }
            "list" => {
                if args.len() == 1 {
                    manager.list_all();
                } else {
                    match args[1] {
                        "done" => manager.list_by_status(Status::Done),
                        "todo" => manager.list_by_status(Status::Todo),
                        "in-progress" => manager.list_by_status(Status::InProgress),
                        "not-done" => manager.list_not_done(),
                        _ => println!("error: filtro de lista no reconocido"),
                    }
                }
            }
            "clear" => {
                // \x1B[2J limpia toda la pantalla
                // \x1B[1;1H mueve el cursor a la esquina superior izquierda
                print!("\x1B[2J\x1B[1;1H\n");
                io::stdout().flush().unwrap();
            }
            "help" => print_help(),
            "exit" | "quit" => {
                println!("cambios guardado\nbye");
                break; // Rompe el ciclo y finaliza el programa
            }
            _ => {
                println!(
                    "error: comando '{}' no reconocido \nescribe 'help'.",
                    command
                );
            }
        }
    }
}

// Conversión de tipos para evitar errores
fn update_status_cli(manager: &mut TaskManager, args: &[&str], status: Status) {
    if args.len() < 2 {
        println!("error: ID de la tarea necesario");
    } else if let Ok(id) = args[1].parse::<i32>() {
        manager.update_status(id, status);
        manager.save_to_file(FILE_PATH);
    } else {
        println!("error: el ID debe ser un número entero");
    }
}

fn print_help() {
    // Comando help
    println!("  --- Comandos ---");
    println!("  add <descripción>          - Añade una nueva tarea");
    println!("  update <id> <descripción>  - Actualiza la descripción");
    println!("  delete <id>                - Elimina una tarea");
    println!("  mark-in-progress <id>      - Marca como en progreso");
    println!("  mark-done <id>             - Marca como realizada");
    println!("  list                       - Lista todas las tareas");
    println!("  list done                  - Lista las tareas realizadas");
    println!("  list todo                  - Lista las tareas por hacer");
    println!("  list in-progress           - Lista las tareas en curso");
    println!("  list not-done              - Lista tareas por hacer y en curso");
    println!("  help                       - Muestra este menú");
    println!("  clear                      - Limpia la pantalla");
    println!("  exit / quit                - Cierra el gestor");
}
