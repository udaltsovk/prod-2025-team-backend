package dto

import image "gitlab.com/drop-table-prod/backend/protos/go/image"

type ImageDTO struct {
	ID          string
	ContentType string
	ImageType   image.ImageType
	Content     []byte
}
