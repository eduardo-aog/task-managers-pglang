# Multi-User Task Managers (Go y Rust)

Este proyecto busca explorar y comparar el desarrollo de herramientas de línea de comandos (CLI) construyendo exactamente el mismo gestor de tareas en dos de los lenguajes de programación de sistemas más populares modernos: **Go** y **Rust**. 

Ambas implementaciones tienen soporte **multiusuario**: los datos (tareas) no se mezclan entre usuarios y cada uno posee su propio archivo JSON de persistencia. Sin embargo, varían ligeramente en la estrategia de interfaz de usuario de consola.

---

## 🛠️ Implementación en Go (Cobra CLI)

La versión en Go está construida utilizando `Cobra`, una de las librerías más potentes de Go para construir aplicaciones CLI. Funciona aceptando argumentos directamente en la llamada al sistema, y cuenta con autenticación mediante contraseñas e inicio de sesión.

### Requisitos previos
- Go 1.20 o superior.

### Compilación y Uso

Para usar el CLI en Go de manera óptima, lo ideal es compilarlo primero creando un archivo binario llamado `task-cli`:

```bash
cd go/
go build -o task-cli main.go
```

Una vez compilado, puedes invocar la aplicación y todos sus comandos usando directamente el binario `./task-cli`:

```bash
# 1. Registrar a un nuevo usuario
./task-cli register <usuario> <contraseña>

# 2. Iniciar sesión en el tracker
./task-cli login <usuario> <contraseña>

# 3. Añadir una tarea (asegúrate de usar comillas para la descripción completa)
./task-cli add "Aprender a usar canales en Go"

# 4. Actualizar una tarea
./task-cli update <id> "Nueva descripción"

# 5. Cambiar el estado de las tareas
./task-cli markInProgress <id>
./task-cli markDone <id>

# 6. Otros comandos interactivos
./task-cli list
./task-cli delete <id>
./task-cli status
./task-cli logout
```

---

## 🦀 Implementación en Rust (REPL Interactivo)

La versión en Rust tiene un enfoque distinto: presenta una interfaz de línea de comandos interactiva de bucle continuo (REPL - Read Eval Print Loop). Una vez ejecutada, la aplicación se mantiene abierta escuchando los comandos del usuario. 

### Requisitos previos
- Rust y `cargo` instalados.

### Uso

Entra en el subdirectorio de rust e inicia la interfaz usando Cargo:

```bash
cd rust/
cargo run
```

Una vez dentro de la sesión interactiva `task-cli>`, debes iniciar sesión para comenzar a enviar comandos. *(Nota: La versión de Rust actualmente no usa contraseñas, solo separa el contexto por nombre).*

```text
========================================
          RusTask Manager
  Escribe 'help' para ver los comandos
========================================
task-cli> login eduardo
Usuario 'eduardo' seleccionado correctamente.

eduardo@task-cli> add Aprender Ownership en Rust
Tarea añadida correctamente (ID: 1)

eduardo@task-cli> mark-in-progress 1
Estado de la tarea 1 actualizado.

eduardo@task-cli> list
[1] Aprender Ownership en Rust | Estatus: InProgress | Creada: 2026-03-26 15:40:02

eduardo@task-cli> help
  --- Comandos ---
  login <nombre>             - Selecciona o crea un usuario (usa su propio JSON)
  add <descripción>          - Añade una nueva tarea
  update <id> <descripción>  - Actualiza la descripción
  delete <id>                - Elimina una tarea
  mark-in-progress <id>      - Marca como en progreso
  mark-done <id>             - Marca como realizada
  list                       - Lista todas las tareas
  list done                  - Lista las tareas realizadas
  list todo                  - Lista las tareas por hacer
  list in-progress           - Lista las tareas en curso
  list not-done              - Lista tareas por hacer y en curso
  help                       - Muestra este menú
  clear                      - Limpia la pantalla
  exit / quit                - Cierra el gestor
```

---

## Persistencia de Datos
Tanto la aplicación de Go como la de Rust generarán y leerán una serie de archivos terminados en `_tasks.json` en sus respectivos directorios (por ejemplo: `eduardo_tasks.json`). Todo se persiste de forma local inmediatamente tras cada instrucción que altera el estado.
