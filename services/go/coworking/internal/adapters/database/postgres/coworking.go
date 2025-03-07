package postgres

import (
	"context"
	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgxpool"
	coworking "gitlab.com/drop-table-prod/backend/protos/go/coworking"
)

type coworkingStorage struct {
	db *pgxpool.Pool
}

func NewCoworkingStorage(db *pgxpool.Pool) *coworkingStorage {
	return &coworkingStorage{
		db: db,
	}
}

const CreateQuery = `
INSERT INTO coworkings (id, name, address) VALUES ($1, $2, $3) RETURNING id;
`

const GetQuery = `
SELECT c.id, c.name, c.address, COUNT(s.id) AS available_seats, COUNT(s.id) AS total_seats
FROM coworkings c
CROSS JOIN seats s
WHERE c.id = $1
GROUP BY c.id, c.name, c.address;
`

const UpdateQuery = `
UPDATE coworkings
SET
    name = COALESCE($2, name),
    address = COALESCE($3, address),
WHERE
    id = $1;
`

const DeleteQuery = `
DELETE FROM coworkings WHERE id = $1;
`

func (s *coworkingStorage) Create(ctx context.Context, dto *coworking.CreateCoworkingRequest) (string, error) {
	var id string
	coworkingUuid, _ := uuid.NewV7()

	err := s.db.QueryRow(ctx, CreateQuery, coworkingUuid, dto.Name, dto.Address).Scan(&id)
	if err != nil {
		return id, err
	}

	return id, nil
}

func (s *coworkingStorage) GetByID(ctx context.Context, id string) (*coworking.CoworkingResponse, error) {
	var res coworking.CoworkingResponse

	err := s.db.QueryRow(ctx, GetQuery, id).Scan(
		&res.Id,
		&res.Name,
		&res.Address,
		&res.AvailableSeats,
		&res.TotalSeats,
	)

	if err != nil {
		return nil, err
	}

	return &res, nil
}

func (s *coworkingStorage) Update(ctx context.Context, dto *coworking.UpdateCoworkingRequest) (*coworking.CoworkingResponse, error) {
	var res coworking.CoworkingResponse

	err := s.db.QueryRow(ctx, UpdateQuery, dto.Name, dto.Address).Scan(
		&res.Id,
		&res.Name,
		&res.Address,
		&res.AvailableSeats,
		&res.TotalSeats,
	)
	if err != nil {
		return nil, err
	}

	return &res, nil
}

func (s *coworkingStorage) Delete(ctx context.Context, id string) error {
	_, err := s.db.Exec(ctx, DeleteQuery, id)
	return err
}
