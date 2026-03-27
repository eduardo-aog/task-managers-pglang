use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    /// Convierte el enum Status a su representación en cadena de texto ("todo", "in-progress", "done").
    fn as_str(&self) -> &str {
        match self {
            Status::Todo => "todo",
            Status::InProgress => "in-progress",
            Status::Done => "done",
        }
    }

    /// Convierte una cadena de texto a su valor correspondiente en el enum Status.
    /// Si el texto no coincide con "in-progress" o "done", retorna Status::Todo por defecto.
    fn from_str(s: &str) -> Self {
        match s {
            "in-progress" => Status::InProgress,
            "done" => Status::Done,
            _ => Status::Todo,
        }
    }
}

pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
}

impl Task {
    /// Crea una nueva tarea con el ID y descripción proporcionados.
    /// Inicializa el estado en `Todo` y establece la fecha de creación y actualización al momento actual.
    pub fn new(id: i32, description: String) -> Self {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Task {
            id,
            description,
            status: Status::Todo,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Actualiza el estado de la tarea y modifica la fecha de actualización (`updated_at`) al momento actual.
    pub fn update_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }

    /// Muestra la información de la tarea en la consola de forma formateada.
    pub fn show(&self) {
        println!(
            "[{}] {} | Estatus: {:?} | Creada: {}",
            self.id, self.description, self.status, self.created_at
        );
    }

    // Serialización a JSON
    /// Serializa la estructura de la tarea a una cadena en formato JSON.
    fn to_json(&self) -> String {
        format!(
            r#"{{"id":{},"description":"{}","status":"{}","created_at":"{}","updated_at":"{}"}}"#,
            self.id,
            self.description.replace('\"', "\\\""),
            self.status.as_str(),
            self.created_at,
            self.updated_at
        )
    }
}

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: i32,
}

impl TaskManager {
    /// Crea una nueva instancia vacía de TaskManager, inicializando el ID de la próxima tarea en 1.
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Todo esto es manejo del archivo JSON

    /// Carga las tareas desde un archivo JSON. Si el archivo no existe o hay un error,
    /// retorna un nuevo TaskManager vacío. También calcula el `next_id` basándose en las tareas cargadas.
    pub fn load_from_file(filename: &str) -> Self {
        let mut manager = TaskManager::new();

        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => return manager,
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            manager.tasks = Self::parse_json(&contents);
            if let Some(max_id) = manager.tasks.iter().map(|t| t.id).max() {
                manager.next_id = max_id + 1;
            }
        }
        manager
    }

    /// Guarda todas las tareas actuales en un archivo en formato JSON.
    /// Crea el archivo si no existe o lo sobrescribe si ya tiene contenido.
    pub fn save_to_file(&self, filename: &str) {
        let json_tasks: Vec<String> = self.tasks.iter().map(|t| t.to_json()).collect();
        let json_string = format!("[\n  {}\n]", json_tasks.join(",\n  "));

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
            .expect("No se pudo abrir o crear el archivo JSON");

        file.write_all(json_string.as_bytes())
            .expect("Error al escribir en el archivo JSON");
    }

    // Parseador manual simple
    /// Analiza (parsea) manualmente una cadena JSON para convertirla en una lista (`Vec`) de tareas.
    /// Divide el string por los delimitadores de objetos y extrae cada campo.
    fn parse_json(json: &str) -> Vec<Task> {
        let mut tasks = Vec::new();
        let parts: Vec<&str> = json.split("},{").collect();

        for part in parts {
            if !part.contains("\"id\":") {
                continue;
            }

            let id = Self::extract_value(part, "\"id\":")
                .parse::<i32>()
                .unwrap_or(0);
            let description = Self::extract_string(part, "\"description\":\"");
            let status_str = Self::extract_string(part, "\"status\":\"");
            let created_at = Self::extract_string(part, "\"created_at\":\"");
            let updated_at = Self::extract_string(part, "\"updated_at\":\"");

            tasks.push(Task {
                id,
                description,
                status: Status::from_str(&status_str),
                created_at,
                updated_at,
            });
        }
        tasks
    }

    /// Función auxiliar para extraer el valor numérico (u otro valor bruto) asociado a una clave dentro de un fragmento JSON.
    fn extract_value(json_part: &str, key: &str) -> String {
        if let Some(start) = json_part.find(key) {
            let value_start = start + key.len();
            if let Some(end) = json_part[value_start..].find(|c| c == ',' || c == '}') {
                return json_part[value_start..value_start + end].trim().to_string();
            }
        }
        String::new()
    }

    /// Función auxiliar para extraer y limpiar una cadena de texto asociada a una clave dentro de un fragmento JSON.
    fn extract_string(json_part: &str, key: &str) -> String {
        if let Some(start) = json_part.find(key) {
            let value_start = start + key.len();
            if let Some(end) = json_part[value_start..].find('\"') {
                return json_part[value_start..value_start + end].to_string();
            }
        }
        String::new()
    }

    // Métodos del Task Manager

    /// Añade una nueva tarea a la lista con la descripción proporcionada.
    /// Asigna automáticamente el ID e incrementa el contador `next_id`.
    pub fn add(&mut self, description: String) {
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        println!("Tarea añadida correctamente (ID: {})", self.next_id);
        self.next_id += 1;
    }

    /// Modifica la descripción de una tarea existente buscando por su ID.
    /// Si la encuentra, actualiza el campo de descripción y la fecha de modificación.
    pub fn update_description(&mut self, id: i32, new_description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.description = new_description;
            task.updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            println!("Descripción de la tarea {} actualizada.", id);
        } else {
            println!("Error: Tarea {} no encontrada.", id);
        }
    }

    /// Modifica el estado de una tarea existente buscando por su ID.
    /// Utiliza el método `update_status` de la tarea para actualizar también la fecha.
    pub fn update_status(&mut self, id: i32, status: Status) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.update_status(status);
            println!("Estado de la tarea {} actualizado.", id);
        } else {
            println!("Error: Tarea {} no encontrada.", id);
        }
    }

    /// Elimina una tarea de la lista buscando por su ID y notifica el resultado de la operación en consola.
    pub fn delete(&mut self, id: i32) {
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() < initial_len {
            println!("Tarea {} eliminada.", id);
        } else {
            println!("Error: Tarea {} no encontrada.", id);
        }
    }

    /// Muestra en la consola todas las tareas registradas en el TaskManager.
    /// Si no hay tareas, informa al usuario.
    pub fn list_all(&self) {
        if self.tasks.is_empty() {
            println!("No hay tareas registradas.");
        } else {
            for task in &self.tasks {
                task.show();
            }
        }
    }

    /// Filtra y muestra por consola las tareas que coincidan con el estado proporcionado.
    pub fn list_by_status(&self, status: Status) {
        let filtered: Vec<&Task> = self.tasks.iter().filter(|t| t.status == status).collect();
        if filtered.is_empty() {
            println!("No hay tareas con estatus: {:?}", status);
        } else {
            for task in filtered {
                task.show();
            }
        }
    }

    /// Filtra y muestra por consola las tareas que no estén en estado completado (`Done`).
    pub fn list_not_done(&self) {
        let filtered: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|t| t.status != Status::Done)
            .collect();
        if filtered.is_empty() {
            println!("¡Todas las tareas están completadas!");
        } else {
            for task in filtered {
                task.show();
            }
        }
    }
}
