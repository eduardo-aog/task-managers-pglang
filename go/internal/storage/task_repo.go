package storage

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"task-cli/internal/models"
	"time"
)

// getFilePath genera el nombre del archivo JSON basado en el nombre de usuario
func getFilePath(username string) string {
	return fmt.Sprintf("tasks_%s.json", username)
}

// loadTasks lee el archivo JSON del usuario y devuelve la lista de tareas
func LoadTasks(username string) ([]models.Task, error) {
	filename := getFilePath(username)

	if _, err := os.Stat(filename); errors.Is(err, os.ErrNotExist) {
		return []models.Task{}, nil
	}

	data, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	var tasks []models.Task

	if err := json.Unmarshal(data, &tasks); err != nil {
		return nil, err
	}

	return tasks, nil
}

// saveTasks guarda la lista de tareas en el archivo JSON del usuario
func saveTasks(username string, tasks []models.Task) error {
	filename := getFilePath(username)

	data, err := json.MarshalIndent(tasks, "", "  ")
	if err != nil {
		return err
	}

	return os.WriteFile(filename, data, 0644)
}

// AddTask crea una nueva tarea, la añade a la lista del usuario y guarda los cambios
func AddTask(username string, description string) (models.Task, error) {
	tasks, err := LoadTasks(username)
	if err != nil {
		return models.Task{}, err
	}

	newID := 1
	if len(tasks) > 0 {
		newID = tasks[len(tasks)-1].ID + 1
	}

	newTask := models.Task{
		ID:          newID,
		Description: description,
		Status:      "todo",
		CreatedAt:   time.Now(),
		UpdatedAt:   time.Now(),
	}

	tasks = append(tasks, newTask)

	if err := saveTasks(username, tasks); err != nil {
		return models.Task{}, err
	}

	return newTask, nil
}

// UpdateTask modifica la descripcion de una tarea existente
func UpdateTask(username string, id int, newDescription string) error {
	tasks, err := LoadTasks(username)
	if err != nil {
		return err
	}

	for i, task := range tasks {
		if task.ID == id {
			tasks[i].Description = newDescription
			tasks[i].UpdatedAt = time.Now()

			return saveTasks(username, tasks)
		}
	}
	return fmt.Errorf("No se encontró la tarea con ID %d", id)
}

// UpdateStatus cambia el estado de una tarea (todo, in-progress, done)
func UpdateStatus(username string, id int, newStatus string) error {
	tasks, err := LoadTasks(username)

	if err != nil {
		return err
	}

	for i, task := range tasks {
		if task.ID == id {
			tasks[i].Status = newStatus
			tasks[i].UpdatedAt = time.Now()
			return saveTasks(username, tasks)
		}
	}
	return fmt.Errorf("No se encontró la tarea con ID %d", id)
}

// DeleteTask elimina una tarea por su ID
func DeleteTask(username string, id int) error {
	tasks, err := LoadTasks(username)
	if err != nil {
		return err
	}

	for i, task := range tasks {
		if task.ID == id {
			tasks = append(tasks[:i], tasks[i+1:]...)
			return saveTasks(username, tasks)
		}
	}
	return fmt.Errorf("No se encontró la tarea con ID %d", id)
}
