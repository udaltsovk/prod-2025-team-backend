package v1

import (
	"context"
	"errors"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgconn"
	coworking "gitlab.com/drop-table-prod/backend/protos/go/coworking"
	"gitlab.com/drop-table-prod/backend/services/go/coworking/cmd/app"
	"gitlab.com/drop-table-prod/backend/services/go/coworking/internal/adapters/database/postgres"
	coworkingServ "gitlab.com/drop-table-prod/backend/services/go/coworking/internal/domain/service"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type CoworkingService interface {
	CreateCoworking(ctx context.Context, dto *coworking.CreateCoworkingRequest) (*coworking.CreateCoworkingResponse, error)
	GetByID(ctx context.Context, id string) (*coworking.CoworkingResponse, error)
	Update(ctx context.Context, dto *coworking.UpdateCoworkingRequest) (*coworking.CoworkingResponse, error)
	Delete(ctx context.Context, id string) error
}

type SeatService interface {
	CreateSeat(ctx context.Context, req *coworking.CreateSeatRequest) (*coworking.SeatResponse, error)
	GetByID(ctx context.Context, id string) (*coworking.SeatResponse, error)
	GetAll(ctx context.Context, limit, offset int) ([]coworking.SeatResponse, error)
	Update(ctx context.Context, seat *coworking.UpdateSeatRequest) (*coworking.SeatResponse, error)
	Delete(ctx context.Context, req *coworking.SeatRequest) error
}

type coworkingHandler struct {
	coworking.UnimplementedCoworkingServer
	coworkingService CoworkingService
	seatService      SeatService
}

func NewCoworkingHandler(app *app.App) *coworkingHandler {
	return &coworkingHandler{
		coworkingService: coworkingServ.NewCoworkingService(postgres.NewCoworkingStorage(app.DB)),
		seatService:      coworkingServ.NewSeatService(postgres.NewSeatStorage(app.DB)),
	}
}

func (h *coworkingHandler) Create(ctx context.Context, request *coworking.CreateCoworkingRequest) (*coworking.CreateCoworkingResponse, error) {
	return h.coworkingService.CreateCoworking(ctx, request)
}

func (h *coworkingHandler) GetByID(ctx context.Context, request *coworking.GetCoworkingByIDRequest) (*coworking.CoworkingResponse, error) {
	res, err := h.coworkingService.GetByID(ctx, *request.Id)
	if errors.Is(err, pgx.ErrNoRows) {
		return nil, status.Error(codes.NotFound, "Not found!")
	}
	return res, err
}

func (h *coworkingHandler) Update(ctx context.Context, request *coworking.UpdateCoworkingRequest) (*coworking.CoworkingResponse, error) {
	res, err := h.coworkingService.Update(ctx, request)
	if errors.Is(err, pgx.ErrNoRows) {
		return nil, status.Error(codes.NotFound, "Not found!")
	}
	return res, err
}

func (h *coworkingHandler) Delete(ctx context.Context, request *coworking.DeleteCoworkingRequest) (*emptypb.Empty, error) {
	err := h.coworkingService.Delete(ctx, *request.Id)
	if err != nil {
		return nil, err
	}

	return &emptypb.Empty{}, nil
}

func (h *coworkingHandler) CreateSeat(ctx context.Context, request *coworking.CreateSeatRequest) (*coworking.SeatResponse, error) {
	res, err := h.seatService.CreateSeat(ctx, request)

	if err != nil {
		var pgErr *pgconn.PgError
		if errors.As(err, &pgErr) {
			if pgErr.Code == "23505" {
				return nil, status.Error(codes.AlreadyExists, pgErr.Message)
			}
		}
		return nil, status.Error(codes.Internal, err.Error())
	}

	return res, nil
}

func (h *coworkingHandler) GetSeat(ctx context.Context, req *coworking.SeatRequest) (*coworking.SeatResponse, error) {
	res, err := h.seatService.GetByID(ctx, *req.Id)
	if errors.Is(err, pgx.ErrNoRows) {
		return nil, status.Error(codes.NotFound, "Not found!")
	}
	return res, err
}

func (h *coworkingHandler) GetSeats(ctx context.Context, req *coworking.GetSeatsRequest) (*coworking.SeatsResponse, error) {
	res, err := h.seatService.GetAll(ctx, int(*req.Limit), int(*req.Offset))
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			return nil, status.Error(codes.NotFound, "Not found!")
		}
		return nil, err
	}
	result := make([]*coworking.SeatResponse, len(res))
	for i, seat := range res {
		result[i] = &seat
	}
	return &coworking.SeatsResponse{Seats: result}, nil
}

func (h *coworkingHandler) UpdateSeat(ctx context.Context, seat *coworking.UpdateSeatRequest) (*coworking.SeatResponse, error) {
	res, err := h.seatService.Update(ctx, seat)
	if errors.Is(err, pgx.ErrNoRows) {
		return nil, status.Error(codes.NotFound, "Not found!")
	}
	return res, err
}

func (h *coworkingHandler) DeleteSeat(ctx context.Context, request *coworking.SeatRequest) (*emptypb.Empty, error) {
	err := h.seatService.Delete(ctx, request)
	if err != nil {
		return nil, err
	}
	return &emptypb.Empty{}, nil
}

func (h *coworkingHandler) Setup(gRPCServer *grpc.Server) {
	coworking.RegisterCoworkingServer(gRPCServer, h)
}
