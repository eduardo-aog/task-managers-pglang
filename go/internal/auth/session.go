package auth

import (
	"encoding/json"
	"errors"
	"os"

	"task-cli/internal/storage"
)

const sessionFile = ".session.json"

type Session struct {
	Username string `json:"username"`
}

// Login verifica las credenciales del usuario y, si son correctas,
// crea un archivo de sesión (.session.json) para mantenerlo conectado.
func Login(username, password string) error {

	err := storage.VerifyCredentials(username, password)
	if err != nil {
		return err
	}

	session := Session{Username: username}
	data, err := json.MarshalIndent(session, "", " ")
	if err != nil {
		return err
	}

	return os.WriteFile(sessionFile, data, 0644)
}

// GetCurrentUser lee el archivo de sesión para determinar qué usuario
// está actualmente conectado. Retorna un error si no hay sesión activa.
func GetCurrentUser() (string, error) {
	if _, err := os.Stat(sessionFile); errors.Is(err, os.ErrNotExist) {
		return "", errors.New("No hay sesion activa. Usa 'task-cli login <usuario> <clave>'")
	}

	data, err := os.ReadFile(sessionFile)
	if err != nil {
		return "", err
	}

	var session Session
	if err := json.Unmarshal(data, &session); err != nil {
		return "", errors.New("Archivo de sesion corrupto, vuelva a hacer login")
	}

	return session.Username, nil
}

// LogOut cierra la sesión actual eliminando el archivo de sesión (.session.json).
// Si el archivo ya no existe, ignora el error y retorna nil.
func LogOut() error {
	err := os.Remove(sessionFile)
	if errors.Is(err, os.ErrNotExist) {
		return nil
	}
	return err
}