package redis

import (
	"context"
	"github.com/redis/go-redis/v9"
	lock "gitlab.com/drop-table-prod/backend/protos/go/seat-lock"
	"time"
)

type seatLockStorage struct {
	db *redis.Client
}

func NewSeatLockStorage(client *redis.Client) *seatLockStorage {
	return &seatLockStorage{
		db: client,
	}
}

func (s *seatLockStorage) Lock(ctx context.Context, req *lock.SetLockRequest) error {
	err := s.db.Set(ctx, *req.UserId, *req.SeatIndex, 5*time.Minute).Err()
	if err != nil {
		return err
	}
	err = s.db.Set(ctx, *req.SeatIndex, *req.UserId, 5*time.Minute).Err()
	if err != nil {
		return err
	}
	return nil
}

func (s *seatLockStorage) GetLockByUserId(ctx context.Context, req *lock.GetLockByUserIdRequest) (*lock.LockResponse, error) {
	value, err := s.db.Get(ctx, *req.UserId).Result()
	if err != nil {
		return &lock.LockResponse{}, err
	}
	status := value == *req.UserId
	return &lock.LockResponse{Status: &status}, nil
}

func (s *seatLockStorage) GetLockBySeatIndex(ctx context.Context, req *lock.GetLockBySeatIndexRequest) (*lock.LockResponse, error) {
	value, err := s.db.Get(ctx, *req.SeatIndex).Result()
	if err != nil {
		return &lock.LockResponse{}, err
	}
	status := value == *req.SeatIndex
	return &lock.LockResponse{Status: &status}, nil
}
