package postgres

import (
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/entity"
)

// Migrations is a list of all gorm migrations for the database.
var Migrations = []interface{}{
	&entity.Reservation{},
}
