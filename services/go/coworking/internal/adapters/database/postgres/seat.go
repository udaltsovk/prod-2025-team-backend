package postgres

import (
	"context"
	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgxpool"
	coworking "gitlab.com/drop-table-prod/backend/protos/go/coworking"
)

type seatStorage struct {
	db *pgxpool.Pool
}

// NewSeatStorage is a function that returns a new instance of seatStorage.
func NewSeatStorage(db *pgxpool.Pool) *seatStorage {
	return &seatStorage{db: db}
}

const CreateSeatQuery = `INSERT INTO seats (id, number, type, capacity, features, cost) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;`

func (s *seatStorage) CreateSeat(ctx context.Context, req *coworking.CreateSeatRequest) (*coworking.SeatResponse, error) {
	var seat coworking.SeatResponse
	seatUUID, _ := uuid.NewV7()
	err := s.db.QueryRow(ctx, CreateSeatQuery, seatUUID, req.Number, req.Type, req.Capacity, req.Features, req.Cost).Scan(
		&seat.Id,
		&seat.Number,
		&seat.Type,
		&seat.Capacity,
		&seat.Features,
		&seat.Cost,
	)
	return &seat, err
}

const GetSeatByIDQuery = `SELECT * FROM seats WHERE id = $1;`

func (s *seatStorage) GetByID(ctx context.Context, id string) (*coworking.SeatResponse, error) {
	var seat coworking.SeatResponse
	err := s.db.QueryRow(ctx, GetSeatByIDQuery, id).Scan(
		&seat.Id,
		&seat.Number,
		&seat.Type,
		&seat.Capacity,
		&seat.Features,
		&seat.Cost,
	)
	return &seat, err
}

const GetAllSeatsQuery = `SELECT * FROM seats LIMIT $1 OFFSET $2;`

func (s *seatStorage) GetAll(ctx context.Context, limit, offset int) ([]coworking.SeatResponse, error) {
	var seats []coworking.SeatResponse
	rows, err := s.db.Query(ctx, GetAllSeatsQuery, limit, offset)
	if err != nil {
		return nil, err
	}

	for rows.Next() {
		var seat coworking.SeatResponse
		err = rows.Scan(
			&seat.Id,
			&seat.Number,
			&seat.Type,
			&seat.Capacity,
			&seat.Features,
			&seat.Cost,
		)
		if err != nil {
			return nil, err
		}
		seats = append(seats, seat)
	}
	return seats, err
}

const UpdateSeatQuery = `
UPDATE seats
SET
    number = COALESCE($2, number),
    type = COALESCE($3, type),
    capacity = COALESCE($4, capacity),
    cost = COALESCE($5, cost)
WHERE id = $1
RETURNING *;
`

func (s *seatStorage) Update(ctx context.Context, req *coworking.UpdateSeatRequest) (*coworking.SeatResponse, error) {
	var seat coworking.SeatResponse
	err := s.db.QueryRow(ctx, UpdateSeatQuery, req.Id, req.Number, req.Type, req.Capacity, req.Cost).Scan(
		&seat.Id,
		&seat.Number,
		&seat.Type,
		&seat.Capacity,
		&seat.Features,
		&seat.Cost,
	)
	return &seat, err
}

const DeleteSeatQuery = `DELETE FROM seats WHERE id = $1;`

func (s *seatStorage) Delete(ctx context.Context, req *coworking.SeatRequest) error {
	_, err := s.db.Exec(ctx, DeleteSeatQuery, req.Id)
	return err
}
