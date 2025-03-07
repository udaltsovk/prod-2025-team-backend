package service

import (
	"context"
	coworking "gitlab.com/drop-table-prod/backend/protos/go/coworking"
)

type seatStorage interface {
	CreateSeat(ctx context.Context, req *coworking.CreateSeatRequest) (*coworking.SeatResponse, error)
	GetByID(ctx context.Context, id string) (*coworking.SeatResponse, error)
	GetAll(ctx context.Context, limit, offset int) ([]coworking.SeatResponse, error)
	Update(ctx context.Context, seat *coworking.UpdateSeatRequest) (*coworking.SeatResponse, error)
	Delete(ctx context.Context, req *coworking.SeatRequest) error
}

type seatService struct {
	storage seatStorage
}

func NewSeatService(storage seatStorage) *seatService {
	return &seatService{storage: storage}
}

func (s *seatService) CreateSeat(ctx context.Context, req *coworking.CreateSeatRequest) (*coworking.SeatResponse, error) {
	return s.storage.CreateSeat(ctx, req)
}

func (s *seatService) GetByID(ctx context.Context, id string) (*coworking.SeatResponse, error) {
	return s.storage.GetByID(ctx, id)
}

func (s *seatService) GetAll(ctx context.Context, limit, offset int) ([]coworking.SeatResponse, error) {
	return s.storage.GetAll(ctx, limit, offset)
}

func (s *seatService) Update(ctx context.Context, seat *coworking.UpdateSeatRequest) (*coworking.SeatResponse, error) {
	return s.storage.Update(ctx, seat)
}

func (s *seatService) Delete(ctx context.Context, req *coworking.SeatRequest) error {
	return s.storage.Delete(ctx, req)
}
