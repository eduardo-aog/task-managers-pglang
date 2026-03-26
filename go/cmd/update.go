/*
Copyright © 2026 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"strconv"
	"task-cli/internal/auth"
	"task-cli/internal/storage"

	"github.com/spf13/cobra"
)

// updateCmd represents the update command
var updateCmd = &cobra.Command{
	Use:   "update [ID] '[Nueva Descripcion]'",
	Short: "Actualiza la descripcion de una tarea existente",
	Args: cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		username, err := auth.GetCurrentUser()
		if err != nil {
			fmt.Println(err)
			return
		}

		id, err := strconv.Atoi(args[0])
		if err != nil {
			fmt.Println("Error: el ID debe ser un numero valido")
			return
		}

		newDescription := args[1]

		err = storage.UpdateTask(username, id, newDescription)
		if err != nil {
			fmt.Println("Error al actualizar:", err)
			return
		}

		fmt.Printf("Tarea %d actualizada correctamente.\n", id)
	},
}

func init() {
	rootCmd.AddCommand(updateCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// updateCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// updateCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
