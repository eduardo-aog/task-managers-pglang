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

// deleteCmd represents the delete command
var deleteCmd = &cobra.Command{
	Use:   "delete [ID]",
	Short: "Elimina una tarea por su ID",
	Args: cobra.ExactArgs(1),
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

		err = storage.DeleteTask(username, id)
		if err != nil {
			fmt.Println("Error al eliminar:", err)
			return
		}

		fmt.Printf("Tarea %d eliminada exitosamente.\n", id)
	},
}

func init() {
	rootCmd.AddCommand(deleteCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// deleteCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// deleteCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
