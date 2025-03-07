package v1

import (
	"context"
	"gitlab.com/drop-table-prod/backend/libs/go/errorz"
	coworking "gitlab.com/drop-table-prod/backend/protos/go/coworking"
	reservation "gitlab.com/drop-table-prod/backend/protos/go/reservation"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/cmd/app"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/adapters/database/postgres"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/entity"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/service"
	coworkingUtils "gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/utils/coworking"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/utils/dotenv"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
	"google.golang.org/protobuf/types/known/timestamppb"
	"log"
	"time"
)

type ReservationService interface {
	Create(ctx context.Context, req reservation.CreateRequest) (*entity.Reservation, error)
	GetByID(ctx context.Context, id string) (*entity.Reservation, error)
	GetAll(ctx context.Context, limit, offset int) ([]entity.Reservation, error)
	GetAllByClient(ctx context.Context, clientID string, limit, offset int) ([]entity.Reservation, error)
	GetBySeat(ctx context.Context, seatID string) ([]entity.Reservation, error)
	GetByDate(ctx context.Context, date time.Time) ([]entity.Reservation, error)
	GetByVisitByDate(ctx context.Context, date time.Time) ([]entity.Reservation, error)
	Update(ctx context.Context, user *entity.Reservation) (*entity.Reservation, error)
	Cancel(ctx context.Context, id string) error
	Visit(ctx context.Context, id string) error
	Delete(ctx context.Context, id string) error
	Exists(ctx context.Context, id string) bool
}

type reservationHandler struct {
	reservation.UnsafeReservationServer
	reservationService ReservationService
	GRPCClient         coworking.CoworkingClient
}

func NewReservationHandler(app *app.App) *reservationHandler {
	reservationStorage := postgres.NewReservationStorage(app.DB)
	conn, err := grpc.Dial(dotenv.GetEnv("ADDRESS_COWORKING", "localhost:50053"), grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Не удалось подключиться к gRPC-серверу: %v", err)
	}
	return &reservationHandler{
		reservationService: service.NewReservationService(reservationStorage),
		GRPCClient:         coworking.NewCoworkingClient(conn),
	}
}

func (h *reservationHandler) Create(ctx context.Context, req *reservation.CreateRequest) (*reservation.ReservationResponse, error) {
	var err error
	var reservationEntity *entity.Reservation
	var cond bool
	cond, err = coworkingUtils.IsSeatExist(ctx, *req.SeatId, h.GRPCClient)
	if err != nil {
		return nil, status.Error(codes.Internal, errorz.Forbidden.Error())
	}
	if !cond {
		return nil, status.Error(codes.NotFound, errorz.SeatNotFound.Error())
	}

	if reservationEntity, err = h.reservationService.Create(ctx, *req); err != nil {
		return nil, err
	}
	return &reservation.ReservationResponse{
		Id:         &reservationEntity.ID,
		ClientId:   &reservationEntity.ClientID,
		SeatId:     &reservationEntity.SeatID,
		StartsAt:   timestamppb.New(reservationEntity.EndsAt),
		EndsAt:     timestamppb.New(reservationEntity.EndsAt),
		IsCanceled: &reservationEntity.IsCanceled,
		IsVisited:  &reservationEntity.IsVisited,
	}, nil
}

func (h *reservationHandler) GetByID(ctx context.Context, req *reservation.GetByIdRequest) (*reservation.ReservationResponse, error) {
	var err error
	var reservationEntity *entity.Reservation
	if reservationEntity, err = h.reservationService.GetByID(ctx, *req.Id); err != nil {
		return nil, err
	}

	if !*req.IsAdmin && reservationEntity.ClientID != *req.ClientId {
		return nil, status.Error(codes.PermissionDenied, errorz.Forbidden.Error())

	}

	return &reservation.ReservationResponse{
		Id:         &reservationEntity.ID,
		ClientId:   &reservationEntity.ClientID,
		SeatId:     &reservationEntity.SeatID,
		StartsAt:   timestamppb.New(reservationEntity.StartsAt),
		EndsAt:     timestamppb.New(reservationEntity.EndsAt),
		IsCanceled: &reservationEntity.IsCanceled,
		IsVisited:  &reservationEntity.IsVisited,
	}, nil
}

func (h *reservationHandler) Update(ctx context.Context, req *reservation.UpdateRequest) (*reservation.ReservationResponse, error) {
	var err error
	var reservationEntity *entity.Reservation
	var cond bool
	cond, err = coworkingUtils.IsSeatExist(ctx, *req.SeatId, h.GRPCClient)
	if err != nil {
		return nil, status.Error(codes.Internal, errorz.Forbidden.Error())
	}
	if !cond {
		return nil, status.Error(codes.NotFound, errorz.SeatNotFound.Error())
	}

	if reservationEntity, err = h.reservationService.GetByID(ctx, *req.Id); err != nil {
		return nil, err
	}

	if !*req.IsAdmin && reservationEntity.ClientID != *req.ClientId {
		return nil, status.Error(codes.PermissionDenied, errorz.Forbidden.Error())
	}

	reservEntity := entity.Reservation{
		ID:       *req.Id,
		ClientID: *req.ClientId,
	}
	if req.SeatId != nil {
		reservEntity.SeatID = *req.SeatId
	} else {
		reservEntity.SeatID = reservationEntity.SeatID
	}
	if req.StartsAt != nil {
		reservEntity.StartsAt = req.StartsAt.AsTime()
	} else {
		reservEntity.StartsAt = reservationEntity.StartsAt
	}
	if req.EndsAt != nil {
		reservEntity.EndsAt = req.EndsAt.AsTime()
	} else {
		reservEntity.EndsAt = reservationEntity.EndsAt
	}
	if req.IsCanceled != nil {
		reservEntity.IsCanceled = *req.IsCanceled
	} else {
		reservEntity.IsCanceled = reservationEntity.IsCanceled
	}
	if req.IsVisited != nil {
		reservEntity.IsVisited = *req.IsVisited
	} else {
		reservEntity.IsVisited = reservationEntity.IsVisited
	}

	if reservationEntity, err = h.reservationService.Update(ctx, &reservEntity); err != nil {
		return nil, err
	}

	return &reservation.ReservationResponse{
		Id:         &reservationEntity.ID,
		ClientId:   &reservationEntity.ClientID,
		SeatId:     &reservationEntity.SeatID,
		StartsAt:   timestamppb.New(reservationEntity.StartsAt),
		EndsAt:     timestamppb.New(reservationEntity.EndsAt),
		IsCanceled: &reservationEntity.IsCanceled,
		IsVisited:  &reservationEntity.IsVisited,
	}, nil
}

func (h *reservationHandler) Delete(ctx context.Context, req *reservation.DeleteRequest) (*emptypb.Empty, error) {
	var err error
	var reservationEntity *entity.Reservation
	if reservationEntity, err = h.reservationService.GetByID(ctx, *req.Id); err != nil {
		return nil, err
	}

	if !*req.IsAdmin && reservationEntity.ClientID != *req.ClientId {
		return nil, status.Error(codes.PermissionDenied, errorz.Forbidden.Error())
	}

	if err = h.reservationService.Delete(ctx, *req.Id); err != nil {
		return nil, err
	}

	return nil, nil
}

func (h *reservationHandler) Cancel(ctx context.Context, req *reservation.DeleteRequest) (*emptypb.Empty, error) {
	var err error
	var reservationEntity *entity.Reservation
	if reservationEntity, err = h.reservationService.GetByID(ctx, *req.Id); err != nil {
		return nil, err
	}

	if !*req.IsAdmin && reservationEntity.ClientID != *req.ClientId {
		return nil, status.Error(codes.PermissionDenied, errorz.Forbidden.Error())
	}

	if err = h.reservationService.Cancel(ctx, *req.Id); err != nil {
		return nil, err
	}

	return nil, nil
}

func (h *reservationHandler) Visit(ctx context.Context, req *reservation.DeleteRequest) (*emptypb.Empty, error) {
	var err error
	var reservationEntity *entity.Reservation
	if reservationEntity, err = h.reservationService.GetByID(ctx, *req.Id); err != nil {
		return nil, err
	}

	if !*req.IsAdmin && reservationEntity.ClientID != *req.ClientId {
		return nil, status.Error(codes.Internal, errorz.Forbidden.Error())
	}

	if err = h.reservationService.Visit(ctx, *req.Id); err != nil {
		return nil, err
	}

	return nil, nil
}

func (h *reservationHandler) GetByClient(ctx context.Context, req *reservation.GetByClientRequest) (*reservation.ReservationsResponse, error) {
	var err error
	var reservEntities []entity.Reservation
	if reservEntities, err = h.reservationService.GetAllByClient(ctx, *req.ClientId, int(*req.Limit), int(*req.Offset)); err != nil {
		return nil, err
	}

	var response []*reservation.ReservationResponse
	for _, reservEntity := range reservEntities {

		response = append(response, &reservation.ReservationResponse{
			Id:         &reservEntity.ID,
			ClientId:   &reservEntity.ClientID,
			SeatId:     &reservEntity.SeatID,
			StartsAt:   timestamppb.New(reservEntity.StartsAt),
			EndsAt:     timestamppb.New(reservEntity.EndsAt),
			IsCanceled: &reservEntity.IsCanceled,
		})
	}
	return &reservation.ReservationsResponse{
		Reservations: response,
	}, nil
}

func (h *reservationHandler) GetBySeat(ctx context.Context, req *reservation.GetBySeatRequest) (*reservation.ReservationsResponse, error) {
	var err error
	var reservEntities []entity.Reservation
	if reservEntities, err = h.reservationService.GetBySeat(ctx, *req.SeatId); err != nil {
		return nil, err
	}

	var response []*reservation.ReservationResponse
	for _, reservEntity := range reservEntities {
		response = append(response, &reservation.ReservationResponse{
			Id:         &reservEntity.ID,
			ClientId:   &reservEntity.ClientID,
			SeatId:     &reservEntity.SeatID,
			StartsAt:   timestamppb.New(reservEntity.StartsAt),
			EndsAt:     timestamppb.New(reservEntity.EndsAt),
			IsCanceled: &reservEntity.IsCanceled,
			IsVisited:  &reservEntity.IsVisited,
		})
	}
	return &reservation.ReservationsResponse{
		Reservations: response,
	}, nil
}
func (h *reservationHandler) Get(ctx context.Context, req *reservation.GetRequest) (*reservation.ReservationsResponse, error) {
	var err error
	var reservEntities []entity.Reservation

	if req.Day != nil || req.Month != nil || req.Year != nil {
		var date time.Time

		if req.Year != nil && req.Month != nil && req.Day != nil {
			date = time.Date(int(*req.Year), time.Month(*req.Month), int(*req.Day), 0, 0, 0, 0, time.UTC)
		} else {
			if req.Year != nil && req.Month != nil {
				date = time.Date(int(*req.Year), time.Month(*req.Month), 1, 0, 0, 0, 0, time.UTC)
			} else {
				if req.Year != nil {
					date = time.Date(int(*req.Year), time.January, 1, 0, 0, 0, 0, time.UTC)
				}
			}
		}

		if reservEntities, err = h.reservationService.GetByDate(ctx, date); err != nil {
			return nil, err
		}
	} else {
		if reservEntities, err = h.reservationService.GetAll(ctx, int(*req.Limit), int(*req.Offset)); err != nil {
			return nil, err
		}
	}

	var response []*reservation.ReservationResponse
	for _, reservEntity := range reservEntities {
		response = append(response, &reservation.ReservationResponse{
			Id:         &reservEntity.ID,
			ClientId:   &reservEntity.ClientID,
			SeatId:     &reservEntity.SeatID,
			StartsAt:   timestamppb.New(reservEntity.StartsAt),
			EndsAt:     timestamppb.New(reservEntity.EndsAt),
			IsCanceled: &reservEntity.IsCanceled,
			IsVisited:  &reservEntity.IsVisited,
		})
	}

	return &reservation.ReservationsResponse{
		Reservations: response,
	}, nil
}

func (h *reservationHandler) GetByVisitByDate(ctx context.Context, req *reservation.GetRequest) (*reservation.ReservationsResponse, error) {
	var err error
	var reservEntities []entity.Reservation
	var date time.Time

	// Парсим дату из запроса
	if req.Year != nil && req.Month != nil && req.Day != nil {
		date = time.Date(int(*req.Year), time.Month(*req.Month), int(*req.Day), 0, 0, 0, 0, time.UTC)
	} else {
		return nil, status.Error(codes.InvalidArgument, "invalid date parameters")
	}

	// Получаем резервации, которые были посещены в указанный день
	if reservEntities, err = h.reservationService.GetByVisitByDate(ctx, date); err != nil {
		return nil, err
	}

	// Формируем ответ
	var response []*reservation.ReservationResponse
	for _, reservEntity := range reservEntities {
		response = append(response, &reservation.ReservationResponse{
			Id:         &reservEntity.ID,
			ClientId:   &reservEntity.ClientID,
			SeatId:     &reservEntity.SeatID,
			StartsAt:   timestamppb.New(reservEntity.StartsAt),
			EndsAt:     timestamppb.New(reservEntity.EndsAt),
			IsCanceled: &reservEntity.IsCanceled,
			IsVisited:  &reservEntity.IsVisited,
		})
	}

	return &reservation.ReservationsResponse{
		Reservations: response,
	}, nil
}

func (h *reservationHandler) Setup(gRPCServer *grpc.Server) {
	reservation.RegisterReservationServer(gRPCServer, h)
}
