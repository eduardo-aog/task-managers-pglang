package storage

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"os"
)

const usersFile = "users.json"

type User struct {
	Username string `json:"username"`
	PasswordHash string `json:"passwordHash"`
}

// HashPassword recibe una contraseña en texto plano, la cifra usando SHA-256
// y devuelve el hash resultante en formato de texto hexadecimal.
func HashPassword(password string) string {
	hash := sha256.Sum256([]byte(password))
	return hex.EncodeToString(hash[:])
}

// LoadUsers lee el archivo JSON de usuarios y devuelve una lista (slice) de estructuras User.
// Si el archivo no existe, devuelve una lista vacía sin error.
func LoadUsers() ([]User, error) {
	if _, err := os.Stat(usersFile); errors.Is(err, os.ErrNotExist) {
		return []User{}, nil
	}

	data, err := os.ReadFile(usersFile)
	if err != nil {
		return nil, err
	}

	var users []User
	if err := json.Unmarshal(data, &users); err != nil {
		return nil, err
	}
	return users, nil
}

// RegisterUser recibe un nombre de usuario y contraseña, crea un nuevo usuario y lo guarda en el archivo.
// Antes de guardarlo, verifica que el usuario no exista previamente y cifra la contraseña ingresada.
func RegisterUser(username, password string) error {
	users, err := LoadUsers()
	if err != nil {
		return err
	}

	for _, u := range users {
		if u.Username == username {
			return errors.New("El usuario ya existe")
		}
	}

	newUser := User {
		Username: username,
		PasswordHash: HashPassword(password),
	}
	users = append(users, newUser)

	data, err := json.MarshalIndent(users, "", " ")
	if err != nil {
		return err
	}
	return os.WriteFile(usersFile, data, 0644)
}

// VerifyCredentials comprueba si las credenciales de inicio de sesión son válidas.
// Para esto convierte la contraseña dada a un hash y la compara con el hash previamente guardado.
func VerifyCredentials(username, password string) error {
	users, err := LoadUsers()
	if err != nil {
		return err
	}

	hashedInput := HashPassword(password)

	for _, u := range users {
		if u.Username == username {
			if u.PasswordHash == hashedInput {
				return nil // Login exitoso
			}
			return errors.New("Datos Incorrectos (password)")
		}
	}

	return errors.New("Datos Incorrectos (user)")
}