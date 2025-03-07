package setup

import (
	"gitlab.com/drop-table-prod/backend/services/go/reservation/cmd/app"
	v1 "gitlab.com/drop-table-prod/backend/services/go/reservation/internal/adapters/controller/api/v1"
	"google.golang.org/grpc"
)

func Setup(app *app.App, gRPCServer *grpc.Server) {
	reservationHandler := v1.NewReservationHandler(app)
	reservationHandler.Setup(gRPCServer)

}
