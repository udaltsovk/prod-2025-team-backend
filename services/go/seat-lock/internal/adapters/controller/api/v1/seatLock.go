package v1

import (
	"context"
	lock "gitlab.com/drop-table-prod/backend/protos/go/seat-lock"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/cmd/app"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/adapters/database/redis"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/domain/service"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/types/known/emptypb"
)

type SeatLockService interface {
	Lock(ctx context.Context, req *lock.SetLockRequest) error
	GetLockByUserId(ctx context.Context, req *lock.GetLockByUserIdRequest) (*lock.LockResponse, error)
	GetLockBySeatIndex(ctx context.Context, req *lock.GetLockBySeatIndexRequest) (*lock.LockResponse, error)
}

type SeatLockHandler struct {
	lock.UnimplementedSeatLockServer
	service SeatLockService
}

func NewSeatLockHandler(app *app.App) *SeatLockHandler {
	return &SeatLockHandler{
		service: service.NewSeatLockService(redis.NewSeatLockStorage(app.DB)),
	}
}

func (h *SeatLockHandler) SetLock(ctx context.Context, req *lock.SetLockRequest) (*emptypb.Empty, error) {
	err := h.service.Lock(ctx, req)
	return &emptypb.Empty{}, err
}

func (h *SeatLockHandler) GetLockByUserID(ctx context.Context, req *lock.GetLockByUserIdRequest) (*lock.LockResponse, error) {
	return h.service.GetLockByUserId(ctx, req)
}

func (h *SeatLockHandler) GetLockBySeatIndex(ctx context.Context, req *lock.GetLockBySeatIndexRequest) (*lock.LockResponse, error) {
	return h.service.GetLockBySeatIndex(ctx, req)
}

func (h *SeatLockHandler) Setup(gRPCServer *grpc.Server) {
	lock.RegisterSeatLockServer(gRPCServer, h)
}
