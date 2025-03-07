package postgres

import (
	"context"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/entity"
	"gorm.io/gorm"
	"time"
)

type reservationStorage struct {
	db *gorm.DB
}

// NewReservationStorage is a function that returns a new instance of reservationStorage.
func NewReservationStorage(db *gorm.DB) *reservationStorage {
	return &reservationStorage{db: db}
}

// Create is a method to create a new Client in database.
func (s *reservationStorage) Create(ctx context.Context, reservation entity.Reservation) (*entity.Reservation, error) {
	err := s.db.WithContext(ctx).Create(&reservation).Error
	return &reservation, err
}

// GetByID is a method that returns an error and a pointer to a Client instance by id.
func (s *reservationStorage) GetByID(ctx context.Context, id string) (*entity.Reservation, error) {
	var reservation *entity.Reservation
	err := s.db.WithContext(ctx).Model(&entity.Reservation{}).Where("id = ?", id).First(&reservation).Error
	return reservation, err
}

// GetAll is a method that returns a slice of pointers to all Client instances.
func (s *reservationStorage) GetAll(ctx context.Context, limit, offset int) ([]entity.Reservation, error) {
	var reservations []entity.Reservation
	err := s.db.WithContext(ctx).Model(&entity.Reservation{}).Limit(limit).Offset(offset).Find(&reservations).Error
	return reservations, err
}

// GetAllByClient is a method that returns a slice of pointers to all Client instances.
func (s *reservationStorage) GetAllByClient(ctx context.Context, clientID string, limit, offset int) ([]entity.Reservation, error) {
	var reservations []entity.Reservation
	err := s.db.WithContext(ctx).Model(&entity.Reservation{}).
		Where("client_id = ?", clientID).
		Limit(limit).Offset(offset).Find(&reservations).Error
	return reservations, err
}

func (s *reservationStorage) GetBySeat(ctx context.Context, seatID string) ([]entity.Reservation, error) {
	var reservations []entity.Reservation
	err := s.db.WithContext(ctx).
		Where("seat_id = ?", seatID).
		Find(&reservations).Error
	return reservations, err
}

func (s *reservationStorage) GetByDate(ctx context.Context, date time.Time) ([]entity.Reservation, error) {
	var reservations []entity.Reservation
	startOfDay := time.Date(date.Year(), date.Month(), date.Day(), 0, 0, 0, 0, date.Location())
	endOfDay := startOfDay.Add(24 * time.Hour)

	err := s.db.WithContext(ctx).
		Where(
			"(starts_at <= ? AND ends_at >= ?) OR "+
				"(starts_at <= ? AND ends_at >= ?) OR "+
				"(starts_at >= ? AND ends_at <= ?)",
			endOfDay, startOfDay,
			startOfDay, endOfDay,
			startOfDay, endOfDay,
		).
		Find(&reservations).Error

	return reservations, err
}

func (s *reservationStorage) GetByVisitByDate(ctx context.Context, visit bool, date time.Time) ([]entity.Reservation, error) {
	var reservations []entity.Reservation
	startOfDay := time.Date(date.Year(), date.Month(), date.Day(), 0, 0, 0, 0, date.Location())
	endOfDay := startOfDay.Add(24 * time.Hour)

	err := s.db.WithContext(ctx).
		Where(
			"((starts_at <= ? AND ends_at >= ?) OR "+
				"(starts_at <= ? AND ends_at >= ?) OR "+
				"(starts_at >= ? AND ends_at <= ?)) AND is_visited = ?",
			endOfDay, startOfDay,
			startOfDay, endOfDay,
			startOfDay, endOfDay,
			visit,
		).
		Find(&reservations).Error

	return reservations, err
}

// Update is a method to update an existing Client in database.
func (s *reservationStorage) Update(ctx context.Context, reservation *entity.Reservation) (*entity.Reservation, error) {
	err := s.db.WithContext(ctx).Model(&entity.Reservation{}).Where("id = ?", reservation.ID).Updates(&reservation).Error
	return reservation, err
}

// Delete is a method to delete an existing Client in database.
func (s *reservationStorage) Delete(ctx context.Context, id string) error {
	return s.db.WithContext(ctx).Unscoped().Delete(&entity.Reservation{}, "id = ?", id).Error
}

func (s *reservationStorage) Exists(ctx context.Context, id string) bool {
	var reservation *entity.Reservation
	return s.db.WithContext(ctx).Model(&entity.Reservation{}).Where("id = ?", id).First(&reservation).Error == nil
}

func (s *reservationStorage) CheckPlace(ctx context.Context, seat string, start, end time.Time) bool {
	var reservation entity.Reservation
	err := s.db.WithContext(ctx).Model(&entity.Reservation{}).
		Where("seat_number = ?", seat).
		Where("is_canceled = ?", false).
		Where(
			"(starts_at <= ? AND ends_at >= ?)"+
				" OR (starts_at <= ? AND ends_at >= ?)"+
				" OR (starts_at >= ? AND ends_at <= ?)",
			end, start,
			start, end,
			start, end,
		).
		First(&reservation).Error

	return err == gorm.ErrRecordNotFound
}
