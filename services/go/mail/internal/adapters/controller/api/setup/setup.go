package setup

import (
	"gitlab.com/drop-table-prod/backend/services/go/mail/cmd/app"
	v1 "gitlab.com/drop-table-prod/backend/services/go/mail/internal/adapters/controller/api/v1"
	"google.golang.org/grpc"
)

func Setup(app *app.App, gRPCServer *grpc.Server) {
	mailHandler := v1.NewMailHandler(app)
	mailHandler.Setup(gRPCServer)
}
