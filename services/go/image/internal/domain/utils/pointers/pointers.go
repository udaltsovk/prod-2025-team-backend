package pointers

func String(s string) *string {
	return &s
}

func UInt32(i uint32) *uint32 {
	return &i
}

func UInt64(i uint64) *uint64 {
	return &i
}
