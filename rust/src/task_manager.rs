enum Status {
    Pendiente,
    EnProceso,
    Hecho,
}

struct Task {
    id: i32,
    name: String,
    status: Status,
    created_at: String
}

struct TaskManager {
    tasks: Vec<Task>,
    id_init: i32
}

