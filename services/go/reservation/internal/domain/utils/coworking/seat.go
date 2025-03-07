package coworking

import (
	"context"
	"errors"
	"log"

	coworking "gitlab.com/drop-table-prod/backend/protos/go/coworking"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func IsSeatExist(ctx context.Context, seatID string, grpcclient coworking.CoworkingClient) (bool, error) {
	_, err := grpcclient.GetSeat(ctx, &coworking.SeatRequest{Id: &seatID})
	if err != nil {
		if st, ok := status.FromError(err); ok {
			log.Printf("gRPC error: %v, code: %v", st.Message(), st.Code())
			if st.Code() == codes.NotFound {
				return false, nil
			}
		}
		log.Printf("Unknown gRPC error: %v", err)
		return false, errors.New("failed to check seat existence")
	}
	return true, nil
}
