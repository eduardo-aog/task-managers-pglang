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

// markDoneCmd represents the markDone command
var markDoneCmd = &cobra.Command{
	Use:   "mark-done [ID]",
	Short: "Marca una tarea como 'done",
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

		err = storage.UpdateStatus(username, id, "done")
		if err != nil {
			fmt.Println("Error:", err)
			return
		}

	fmt.Printf("Tarea %d Completada.\n", id)
	},
}

func init() {
	rootCmd.AddCommand(markDoneCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// markDoneCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// markDoneCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
