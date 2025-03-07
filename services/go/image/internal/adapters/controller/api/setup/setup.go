package setup

import (
	"gitlab.com/drop-table-prod/backend/services/go/image/cmd/app"
	v1 "gitlab.com/drop-table-prod/backend/services/go/image/internal/adapters/controller/api/v1"
	"google.golang.org/grpc"
)

func Setup(app *app.App, gRPCServer *grpc.Server) {
	imageHandler := v1.NewImageHandler(app)
	imageHandler.Setup(gRPCServer)
}
