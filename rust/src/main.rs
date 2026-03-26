mod cli;
mod task_manager;

fn main() {
    // Delegamos toda la lógica de los argumentos al módulo CLI
    cli::run();
}
