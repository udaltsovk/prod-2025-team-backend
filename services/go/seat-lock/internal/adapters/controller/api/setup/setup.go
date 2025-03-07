package setup

import (
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/cmd/app"
	v1 "gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/adapters/controller/api/v1"
	"google.golang.org/grpc"
)

func Setup(app *app.App, gRPCServer *grpc.Server) {
	seatLockHandler := v1.NewSeatLockHandler(app)
	seatLockHandler.Setup(gRPCServer)
}
