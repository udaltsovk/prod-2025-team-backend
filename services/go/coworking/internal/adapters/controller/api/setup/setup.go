package setup

import (
	"gitlab.com/drop-table-prod/backend/services/go/coworking/cmd/app"
	v1 "gitlab.com/drop-table-prod/backend/services/go/coworking/internal/adapters/controller/api/v1"
	"google.golang.org/grpc"
)

func Setup(app *app.App, gRPCServer *grpc.Server) {
	coworkingHandler := v1.NewCoworkingHandler(app)
	coworkingHandler.Setup(gRPCServer)
}
