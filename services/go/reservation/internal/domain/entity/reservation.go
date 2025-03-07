package entity

import (
	"time"
)

type Reservation struct {
	CreatedAt time.Time `json:"-"`
	UpdatedAt time.Time `json:"-"`

	ID         string    `json:"id" gorm:"primaryKey;not null"`
	ClientID   string    `json:"client_id" gorm:"index,not null"`
	SeatID     string    `json:"seat_number" gorm:"index,not null"`
	StartsAt   time.Time `json:"starts_at" gorm:"not null"`
	EndsAt     time.Time `json:"ends_at" gorm:"not null"`
	IsCanceled bool      `json:"is_canceled" gorm:"not null,default:false"`
	IsVisited  bool      `json:"is_visited" gorm:"not null,default:false"`
}
