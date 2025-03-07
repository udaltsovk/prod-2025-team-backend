package service

import (
	"context"
	"time"

	"github.com/google/uuid"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	reservation "gitlab.com/drop-table-prod/backend/protos/go/reservation"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/entity"
)

type reservationStorage interface {
	Create(ctx context.Context, reservation entity.Reservation) (*entity.Reservation, error)
	GetByID(ctx context.Context, id string) (*entity.Reservation, error)
	GetAll(ctx context.Context, limit, offset int) ([]entity.Reservation, error)
	GetAllByClient(ctx context.Context, clientID string, limit, offset int) ([]entity.Reservation, error)
	GetBySeat(ctx context.Context, seatID string) ([]entity.Reservation, error)
	GetByDate(ctx context.Context, date time.Time) ([]entity.Reservation, error)
	GetByVisitByDate(ctx context.Context, visit bool, date time.Time) ([]entity.Reservation, error)
	Update(ctx context.Context, reservation *entity.Reservation) (*entity.Reservation, error)
	Delete(ctx context.Context, id string) error
	Exists(ctx context.Context, id string) bool
	CheckPlace(ctx context.Context, seat string, start, end time.Time) bool
}

type reservationService struct {
	storage reservationStorage
}

func NewReservationService(storage reservationStorage) *reservationService {
	return &reservationService{storage: storage}
}

func (s *reservationService) Create(ctx context.Context, req reservation.CreateRequest) (*entity.Reservation, error) {
	if !s.storage.CheckPlace(ctx, *req.SeatId, req.StartsAt.AsTime(), req.EndsAt.AsTime()) {
		return nil, status.Errorf(codes.ResourceExhausted, "seat is already booked")
	}

	res := entity.Reservation{
		ID:         uuid.NewString(),
		ClientID:   *req.ClientId,
		SeatID:     *req.SeatId,
		StartsAt:   req.StartsAt.AsTime(),
		EndsAt:     req.EndsAt.AsTime(),
		IsCanceled: false,
		IsVisited:  false,
	}

	newRes, err := s.storage.Create(ctx, res)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to create reservation: %v", err)
	}
	return newRes, nil
}

func (s *reservationService) GetByID(ctx context.Context, id string) (*entity.Reservation, error) {
	if !s.storage.Exists(ctx, id) {
		return nil, status.Errorf(codes.NotFound, "reservation not found")
	}
	res, err := s.storage.GetByID(ctx, id)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to get reservation: %v", err)
	}
	return res, nil
}

func (s *reservationService) GetAll(ctx context.Context, limit, offset int) ([]entity.Reservation, error) {
	res, err := s.storage.GetAll(ctx, limit, offset)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to get reservations: %v", err)
	}
	return res, nil
}

func (s *reservationService) GetAllByClient(ctx context.Context, clientID string, limit, offset int) ([]entity.Reservation, error) {
	res, err := s.storage.GetAllByClient(ctx, clientID, limit, offset)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to get client reservations: %v", err)
	}
	return res, nil
}

func (s *reservationService) GetBySeat(ctx context.Context, seatID string) ([]entity.Reservation, error) {
	// Получаем все бронирования по сиденью
	reservations, err := s.storage.GetBySeat(ctx, seatID)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to get reservations by seat: %v", err)
	}

	if len(reservations) == 0 {
		return nil, status.Errorf(codes.NotFound, "no reservations found for seat ID: %s", seatID)
	}

	return reservations, nil
}

func (s *reservationService) GetByDate(ctx context.Context, date time.Time) ([]entity.Reservation, error) {
	reservations, err := s.storage.GetByDate(ctx, date)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to get reservations by date: %v", err)
	}

	return reservations, nil
}

func (s *reservationService) GetByVisitByDate(ctx context.Context, date time.Time) ([]entity.Reservation, error) {
	reservations, err := s.storage.GetByVisitByDate(ctx, true, date)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to get visited reservations by date: %v", err)
	}

	if len(reservations) == 0 {
		return nil, status.Errorf(codes.NotFound, "no visited reservations found for date: %v", date)
	}

	return reservations, nil
}

func (s *reservationService) Update(ctx context.Context, reservation *entity.Reservation) (*entity.Reservation, error) {
	if !s.storage.Exists(ctx, reservation.ID) {
		return nil, status.Errorf(codes.NotFound, "reservation not found")
	}

	if !s.storage.CheckPlace(ctx, reservation.SeatID, reservation.StartsAt, reservation.EndsAt) {
		return nil, status.Errorf(codes.ResourceExhausted, "seat is already booked")
	}

	updatedRes, err := s.storage.Update(ctx, reservation)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "failed to update reservation: %v", err)
	}
	return updatedRes, nil
}

func (s *reservationService) Cancel(ctx context.Context, id string) error {
	if !s.storage.Exists(ctx, id) {
		return status.Errorf(codes.NotFound, "reservation not found")
	}
	_, err := s.storage.Update(ctx, &entity.Reservation{ID: id, IsCanceled: true})
	if err != nil {
		return status.Errorf(codes.Internal, "failed to cancel reservation: %v", err)
	}
	return nil
}

func (s *reservationService) Visit(ctx context.Context, id string) error {
	if !s.storage.Exists(ctx, id) {
		return status.Errorf(codes.NotFound, "reservation not found")
	}
	_, err := s.storage.Update(ctx, &entity.Reservation{ID: id, IsVisited: true})
	if err != nil {
		return status.Errorf(codes.Internal, "failed to mark reservation as visited: %v", err)
	}
	return nil
}

func (s *reservationService) Delete(ctx context.Context, id string) error {
	if !s.storage.Exists(ctx, id) {
		return status.Errorf(codes.NotFound, "reservation not found")
	}
	err := s.storage.Delete(ctx, id)
	if err != nil {
		return status.Errorf(codes.Internal, "failed to delete reservation: %v", err)
	}
	return nil
}

func (s *reservationService) Exists(ctx context.Context, id string) bool {
	return s.storage.Exists(ctx, id)
}
