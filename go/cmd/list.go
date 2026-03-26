/*
Copyright © 2026 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"os"
	"text/tabwriter"

	"task-cli/internal/auth"
	"task-cli/internal/storage"

	"github.com/spf13/cobra"
)

// listCmd represents the list command
var listCmd = &cobra.Command{
	Use:   "list [status]",
	Short: "Muestra tus tareas (filtros opcionales: todo, in-progress, done)",
	Args: cobra.MaximumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		username, err := auth.GetCurrentUser()
		if err != nil {
			fmt.Println(err)
			return
		}

		tasks, err := storage.LoadTasks(username)
		if err != nil {
			fmt.Println("No tienes tareas. Usa 'task-cli add' para crear una")
			return
		}

		filter := ""
		if len(args) == 1 {
			filter = args[0]

			if filter != "todo" && filter != "in-progress" && filter != "done" {
				fmt.Println("Error: El filtro debe ser 'todo', 'in-progress' o 'done'")
				return
			}
		}

		w := tabwriter.NewWriter(os.Stdout, 0, 0, 3, ' ', 0)
		fmt.Fprintln(w, "ID\tESTADO\tDESCRIPCIÓN\tCREADA")
		fmt.Fprintln(w, "--\t------\t-----------\t------")

		count := 0
		for _, t := range tasks {
			if filter != "" && t.Status != filter {
				continue
			}

			fecha := t.CreatedAt.Format("02-Jan-2006")

			fmt.Fprintf(w, "%d\t[%s]\t%s\t%s\n", t.ID, t.Status, t.Description, fecha)
			count++
		}

		w.Flush()


		if count == 0 && filter != "" {
			fmt.Printf("\nNo tienes tareas con el estado '%s'.\n", filter)		}
	},
}

func init() {
	rootCmd.AddCommand(listCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// listCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// listCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
