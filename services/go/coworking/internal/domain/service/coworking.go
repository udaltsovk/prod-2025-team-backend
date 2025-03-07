package service

import (
	"context"
	coworking "gitlab.com/drop-table-prod/backend/protos/go/coworking"
	"gitlab.com/drop-table-prod/backend/services/go/coworking/internal/domain/utils/pointers"
)

type CoworkingStorage interface {
	Create(ctx context.Context, dto *coworking.CreateCoworkingRequest) (string, error)
	GetByID(ctx context.Context, id string) (*coworking.CoworkingResponse, error)
	Update(ctx context.Context, dto *coworking.UpdateCoworkingRequest) (*coworking.CoworkingResponse, error)
	Delete(ctx context.Context, id string) error
}

type CoworkingService struct {
	coworkingStorage CoworkingStorage
}

func NewCoworkingService(cs CoworkingStorage) *CoworkingService {
	return &CoworkingService{
		coworkingStorage: cs,
	}
}

func (s *CoworkingService) CreateCoworking(ctx context.Context, dto *coworking.CreateCoworkingRequest) (*coworking.CreateCoworkingResponse, error) {
	result, err := s.coworkingStorage.Create(ctx, dto)
	if err != nil {
		return nil, err
	}

	return &coworking.CreateCoworkingResponse{Id: pointers.String(result)}, nil
}

func (s *CoworkingService) GetByID(ctx context.Context, id string) (*coworking.CoworkingResponse, error) {
	return s.coworkingStorage.GetByID(ctx, id)
}

func (s *CoworkingService) Update(ctx context.Context, dto *coworking.UpdateCoworkingRequest) (*coworking.CoworkingResponse, error) {
	return s.coworkingStorage.Update(ctx, dto)
}

func (s *CoworkingService) Delete(ctx context.Context, id string) error {
	return s.coworkingStorage.Delete(ctx, id)
}
