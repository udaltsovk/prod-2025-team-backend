package main

import (
	"gitlab.com/drop-table-prod/backend/services/go/reservation/cmd/app"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/adapters/config"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/adapters/controller/api/setup"
)

func main() {
	appConfig := config.Configure()
	mainApp := app.New(appConfig)

	setup.Setup(mainApp, mainApp.GRPCServer)
	mainApp.Start()
	defer mainApp.Stop()
}
