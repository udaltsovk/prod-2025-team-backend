package service

import (
	"context"
	"github.com/golang/protobuf/ptypes/empty"
	image "gitlab.com/drop-table-prod/backend/protos/go/image"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/domain/dto"
)

type ImageStorage interface {
	Upload(ctx context.Context, req *dto.ImageDTO) (*image.UploadImageResponse, error)
	GetImage(ctx context.Context, req *image.GetImageRequest) (*image.GetImageResponse, error)
	DeleteImage(ctx context.Context, req *image.DeleteImageRequest) (*empty.Empty, error)
}

type imageService struct {
	storage ImageStorage
}

func NewImageService(storage ImageStorage) *imageService {
	return &imageService{
		storage: storage,
	}
}

func (s *imageService) Upload(ctx context.Context, req *dto.ImageDTO) (*image.UploadImageResponse, error) {
	return s.storage.Upload(ctx, req)
}

func (s *imageService) GetImage(ctx context.Context, req *image.GetImageRequest) (*image.GetImageResponse, error) {
	return s.storage.GetImage(ctx, req)
}

func (s *imageService) DeleteImage(ctx context.Context, req *image.DeleteImageRequest) (*empty.Empty, error) {
	return s.storage.DeleteImage(ctx, req)
}
