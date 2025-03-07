package main

import (
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/cmd/app"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/adapters/config"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/adapters/controller/api/setup"
)

func main() {
	appConfig := config.Configure()
	mainApp := app.New(appConfig)

	setup.Setup(mainApp, mainApp.GRPCServer)
	mainApp.Start()
	defer mainApp.Stop()
}
