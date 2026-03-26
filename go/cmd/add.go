/*
Copyright © 2026 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"task-cli/internal/auth"
	"task-cli/internal/storage"

	"github.com/spf13/cobra"
)

// addCmd represents the add command
var addCmd = &cobra.Command{
	Use:   "add '[descripcion]'",
	Short: "Añade una nueva tarea a tu lista",
	Args: cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		username, err := auth.GetCurrentUser()
		if err != nil {
			fmt.Println(err)
			return
		}

		descripcion := args[0]

		task, err := storage.AddTask(username, descripcion)
		if err != nil {
			fmt.Println("Error al guardar la tarea:", err)
			return
		}

		fmt.Printf("Tarea agregada exitosamente (ID: %d)\n", task.ID)
	},
}

func init() {
	rootCmd.AddCommand(addCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// addCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// addCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
