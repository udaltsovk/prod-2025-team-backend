package service

import (
	"context"
	lock "gitlab.com/drop-table-prod/backend/protos/go/seat-lock"
)

type SeatLockStorage interface {
	Lock(ctx context.Context, req *lock.SetLockRequest) error
	GetLockByUserId(ctx context.Context, req *lock.GetLockByUserIdRequest) (*lock.LockResponse, error)
	GetLockBySeatIndex(ctx context.Context, req *lock.GetLockBySeatIndexRequest) (*lock.LockResponse, error)
}

type SeatLockService struct {
	storage SeatLockStorage
}

func NewSeatLockService(storage SeatLockStorage) *SeatLockService {
	return &SeatLockService{
		storage: storage,
	}
}

func (s *SeatLockService) Lock(ctx context.Context, req *lock.SetLockRequest) error {
	return s.storage.Lock(ctx, req)
}

func (s *SeatLockService) GetLockByUserId(ctx context.Context, req *lock.GetLockByUserIdRequest) (*lock.LockResponse, error) {
	return s.storage.GetLockByUserId(ctx, req)
}

func (s *SeatLockService) GetLockBySeatIndex(ctx context.Context, req *lock.GetLockBySeatIndexRequest) (*lock.LockResponse, error) {
	return s.storage.GetLockBySeatIndex(ctx, req)
}
