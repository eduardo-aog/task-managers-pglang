mod cli;
mod task_manager;

/// Punto de entrada principal de la aplicación.
/// Simplemente delega toda la ejecución y lógica interactiva al módulo `cli`.
fn main() {
    // Delegamos toda la lógica de los argumentos al módulo CLI
    cli::run();
}
