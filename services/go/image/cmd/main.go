package main

import (
	"gitlab.com/drop-table-prod/backend/services/go/image/cmd/app"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/adapters/config"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/adapters/controller/api/setup"
)

func main() {
	appConfig := config.Configure()
	mainApp := app.New(appConfig)

	setup.Setup(mainApp, mainApp.GRPCServer)
	mainApp.Start()
	defer mainApp.Stop()
}
