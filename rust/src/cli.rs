use crate::task_manager::{Status, TaskManager};
use std::io::{self, Write};

/// Función principal que arranca la interfaz de línea de comandos (CLI).
/// Muestra un mensaje de bienvenida y entra en un bucle infinito ("loop")
/// esperando y procesando los comandos del usuario (con soporte multi-usuario).
pub fn run() {
    let mut current_user: Option<String> = None;
    let mut manager = TaskManager::new(); // Inicia vacío, cargará al hacer login

    println!("========================================");
    println!("          RusTask Manager");
    println!("  Escribe 'help' para ver los comandos");
    println!("========================================");

    loop {
        // Para mantener el programa en ejecución
        if let Some(ref user) = current_user {
            print!("{}@task-cli> ", user);
        } else {
            print!("task-cli> ");
        }
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

        // 1. Comandos que no requieren un usuario activo
        match command {
            "login" | "user" => {
                if args.len() < 2 {
                    println!("error: nombre de usuario necesario (ejemplo: login eduardo)");
                } else {
                    let user = args[1].to_string();
                    let file_path = format!("{}_tasks.json", user);
                    manager = TaskManager::load_from_file(&file_path);
                    current_user = Some(user.clone());
                    println!("Usuario '{}' seleccionado correctamente.", user);
                }
                continue;
            }
            "help" => {
                print_help();
                continue;
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H\n");
                io::stdout().flush().unwrap();
                continue;
            }
            "exit" | "quit" => {
                println!("cambios guardados\nbye");
                break;
            }
            _ => {}
        }

        // 2. Verificamos si hay un usuario activo para el resto de comandos
        let user = match &current_user {
            Some(u) => u,
            None => {
                println!("error: debes seleccionar un usuario primero. Usa: login <nombre>");
                continue;
            }
        };

        let file_path = format!("{}_tasks.json", user);

        // 3. Comandos de gestión de tareas
        match command {
            "add" => {
                if args.len() < 2 {
                    println!("error: descripción de la tarea necesaria");
                } else {
                    let description = args[1..].join(" ");
                    manager.add(description);
                    manager.save_to_file(&file_path);
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
                    manager.save_to_file(&file_path);
                } else {
                    println!("error: el ID debe ser un número entero");
                }
            }
            "delete" => {
                if args.len() < 2 {
                    println!("error: ID de la tarea a eliminar necesario");
                } else if let Ok(id) = args[1].parse::<i32>() {
                    manager.delete(id);
                    manager.save_to_file(&file_path);
                } else {
                    println!("error: el ID debe ser un número entero");
                }
            }
            "mark-in-progress" => {
                update_status_cli(&mut manager, &args, Status::InProgress, &file_path);
            }
            "mark-done" => {
                update_status_cli(&mut manager, &args, Status::Done, &file_path);
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
/// Función auxiliar para actualizar el estado de una tarea desde la consola.
/// Verifica que se haya suministrado el ID correcto, intenta convertirlo a número,
/// y si tiene éxito, realiza la actualización en memoria y guarda en disco.
fn update_status_cli(manager: &mut TaskManager, args: &[&str], status: Status, file_path: &str) {
    if args.len() < 2 {
        println!("error: ID de la tarea necesario");
    } else if let Ok(id) = args[1].parse::<i32>() {
        manager.update_status(id, status);
        manager.save_to_file(file_path);
    } else {
        println!("error: el ID debe ser un número entero");
    }
}

/// Muestra en la consola la lista de comandos disponibles y su sintaxis.
/// Sirve como menú de ayuda y referencia rápida para el usuario.
fn print_help() {
    // Comando help
    println!("  --- Comandos ---");
    println!("  login <nombre>             - Selecciona o crea un usuario (usa su propio JSON)");
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
