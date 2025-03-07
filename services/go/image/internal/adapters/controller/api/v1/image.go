package v1

import (
	"context"
	"errors"
	"github.com/golang/protobuf/ptypes/empty"
	"gitlab.com/drop-table-prod/backend/libs/go/errorz"
	image "gitlab.com/drop-table-prod/backend/protos/go/image"
	"gitlab.com/drop-table-prod/backend/services/go/image/cmd/app"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/adapters/s3/minio"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/domain/dto"
	imageServ "gitlab.com/drop-table-prod/backend/services/go/image/internal/domain/service"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/domain/utils/dotenv"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
	"io"
)

type imageService interface {
	Upload(ctx context.Context, req *dto.ImageDTO) (*image.UploadImageResponse, error)
	GetImage(ctx context.Context, req *image.GetImageRequest) (*image.GetImageResponse, error)
	DeleteImage(ctx context.Context, req *image.DeleteImageRequest) (*empty.Empty, error)
}

type imageHandler struct {
	image.UnimplementedImageServer
	imageService imageService
}

func NewImageHandler(app *app.App) *imageHandler {
	return &imageHandler{
		imageService: imageServ.NewImageService(minio.NewImageStorage(
			app.Minio,
			dotenv.GetEnv("COWORKING_BUCKET", "coworking-images"),
			dotenv.GetEnv("SEAT_BUCKET", "seat-images"),
			dotenv.GetEnv("FEATURES_BUCKET", "feature-images"),
			dotenv.GetEnv("AVATAR_BUCKET", "avatar-images"),
		)),
	}
}

func (h *imageHandler) UploadImage(stream grpc.ClientStreamingServer[image.UploadImageRequest, image.UploadImageResponse]) error {
	var imageDTO dto.ImageDTO
	for {
		data, err := stream.Recv()
		if err == io.EOF {
			resp, errUpload := h.imageService.Upload(stream.Context(), &imageDTO)
			if errUpload != nil {
				return status.Error(codes.Internal, errUpload.Error())
			}
			return stream.SendAndClose(resp)
		}
		if err != nil {
			return status.Error(codes.Internal, err.Error())
		}

		switch data.Data.(type) {
		case *image.UploadImageRequest_Metadata:
			imageDTO.ID = *data.Data.(*image.UploadImageRequest_Metadata).Metadata.Id
			imageDTO.ContentType = *data.Data.(*image.UploadImageRequest_Metadata).Metadata.ContentType
			imageDTO.ImageType = *data.Data.(*image.UploadImageRequest_Metadata).Metadata.ImageType
		case *image.UploadImageRequest_Content:
			imageDTO.Content = append(imageDTO.Content, data.Data.(*image.UploadImageRequest_Content).Content...)
		}
	}
}

func (h *imageHandler) GetImage(request *image.GetImageRequest, respStream grpc.ServerStreamingServer[image.GetImageResponse]) error {
	res, err := h.imageService.GetImage(respStream.Context(), request)
	if err != nil {
		if errors.Is(err, errorz.NotFound) {
			return status.Error(codes.NotFound, err.Error())
		}
		return status.Error(codes.Internal, err.Error())
	}

	const chunkSize = 4096 // Можно регулировать
	content := res.Content
	totalBytes := len(content)

	for offset := 0; offset < totalBytes; offset += chunkSize {
		end := offset + chunkSize
		if end > totalBytes {
			end = totalBytes
		}

		chunk := content[offset:end]

		if sendErr := respStream.Send(&image.GetImageResponse{
			Content: chunk,
		}); sendErr != nil {
			return sendErr
		}
	}

	return nil
}

func (h *imageHandler) DeleteImage(ctx context.Context, request *image.DeleteImageRequest) (*emptypb.Empty, error) {
	emptyMessage, err := h.imageService.DeleteImage(ctx, request)
	if err != nil {
		return nil, err
	}

	return emptyMessage, nil
}

func (h *imageHandler) Setup(gRPCServer *grpc.Server) {
	image.RegisterImageServer(gRPCServer, h)
}
