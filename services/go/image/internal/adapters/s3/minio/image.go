package minio

import (
	"bytes"
	"context"
	"fmt"
	"github.com/golang/protobuf/ptypes/empty"
	"github.com/minio/minio-go/v7"
	"gitlab.com/drop-table-prod/backend/libs/go/errorz"
	image "gitlab.com/drop-table-prod/backend/protos/go/image"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/domain/dto"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/domain/utils/pointers"
	"io"
)

type imageStorage struct {
	client          *minio.Client
	coworkingBucket string
	seatBucket      string
	featureBucket   string
	avatarBucket    string
}

func NewImageStorage(client *minio.Client, coworkingBucket, seatBucket, featureBucket, avatarBucket string) *imageStorage {
	return &imageStorage{
		client:          client,
		coworkingBucket: coworkingBucket,
		seatBucket:      seatBucket,
		featureBucket:   featureBucket,
		avatarBucket:    avatarBucket,
	}
}

// TODO secure routes for avatars

func (s *imageStorage) Upload(ctx context.Context, req *dto.ImageDTO) (*image.UploadImageResponse, error) {
	var bucket string
	switch req.ImageType {
	case image.ImageType_AVATAR:
		bucket = s.avatarBucket
	case image.ImageType_COWORKING:
		bucket = s.coworkingBucket
	case image.ImageType_SEAT:
		bucket = s.seatBucket
	case image.ImageType_FEATURE:
		bucket = s.featureBucket
	}

	filename := fmt.Sprintf("%s-%s", req.ImageType.String(), req.ID)

	_, err := s.client.PutObject(ctx, bucket, filename, bytes.NewReader(req.Content), -1, minio.PutObjectOptions{
		ContentType: req.ContentType,
	})

	if err != nil {
		return nil, err
	}

	return &image.UploadImageResponse{Filename: pointers.String(filename)}, nil
}

func (s *imageStorage) GetImage(ctx context.Context, req *image.GetImageRequest) (*image.GetImageResponse, error) {
	var bucket string
	switch *req.ImageType {
	case image.ImageType_AVATAR:
		bucket = s.avatarBucket
	case image.ImageType_COWORKING:
		bucket = s.coworkingBucket
	case image.ImageType_SEAT:
		bucket = s.seatBucket
	case image.ImageType_FEATURE:
		bucket = s.featureBucket
	}

	filename := fmt.Sprintf("%s-%s", req.ImageType.String(), *req.Id)

	_, err := s.client.StatObject(ctx, bucket, filename, minio.StatObjectOptions{})
	if err != nil {
		return nil, errorz.NotFound
	}

	object, err := s.client.GetObject(ctx, bucket, filename, minio.GetObjectOptions{})
	defer func(object *minio.Object) {
		_ = object.Close()
	}(object)

	if err != nil {
		return nil, err
	}

	buffer, err := io.ReadAll(object)
	if err != nil {
		return nil, err
	}

	return &image.GetImageResponse{Content: buffer}, nil
}

// TODO secure routes

func (s *imageStorage) DeleteImage(ctx context.Context, req *image.DeleteImageRequest) (*empty.Empty, error) {
	var bucket string
	switch *req.ImageType {
	case image.ImageType_AVATAR:
		bucket = s.avatarBucket
	case image.ImageType_COWORKING:
		bucket = s.coworkingBucket
	case image.ImageType_SEAT:
		bucket = s.seatBucket
	case image.ImageType_FEATURE:
		bucket = s.featureBucket
	}

	filename := fmt.Sprintf("%s-%s", req.ImageType.String(), *req.Id)

	_, err := s.client.StatObject(ctx, bucket, filename, minio.StatObjectOptions{})
	if err != nil {
		return nil, errorz.NotFound
	}

	err = s.client.RemoveObject(ctx, bucket, filename, minio.RemoveObjectOptions{})
	if err != nil {
		return nil, err
	}

	return &empty.Empty{}, nil
}
