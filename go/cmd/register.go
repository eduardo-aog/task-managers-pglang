/*
Copyright © 2026 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"task-cli/internal/storage"

	"github.com/spf13/cobra"
)

// registerCmd represents the register command
var registerCmd = &cobra.Command{
	Use:   "register [usuario] [contraseña]",
	Short: "Crea una nueva cuenta de usuario",
	Args: cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		username := args[0]
		password := args[1]

		err := storage.RegisterUser(username, password)
		if err != nil {
			fmt.Println("Error al registrar:", err)
			return
		}

		fmt.Printf("¡Usuario '%s' registrado exitosamente!\n", username)
		fmt.Printf("Ahora puedes iniciar sesión ejecutando: task-cli login %s %s\n", username, password)

	},
}

func init() {
	rootCmd.AddCommand(registerCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// registerCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// registerCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
