package errorz

import "errors"

var (
	coworkingNotSent = errors.New("your coworking didn't sent")
	SeatNotFound     = errors.New("seat not found")
	Forbidden        = errors.New("forbidden")
	NotFound         = errors.New("not found")
	Conflict         = errors.New("duplicate key")
)
