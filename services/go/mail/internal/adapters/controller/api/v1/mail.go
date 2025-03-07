package v1

import (
	"context"
	mail "gitlab.com/drop-table-prod/backend/protos/go/mail"
	"gitlab.com/drop-table-prod/backend/services/go/mail/cmd/app"
	mailServ "gitlab.com/drop-table-prod/backend/services/go/mail/internal/domain/service"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/emptypb"
)

type MailService interface {
	SendEmail(to []string, topic, body string) error
}

type mailHandler struct {
	mail.UnimplementedMailServiceServer
	mailservice MailService
}

func NewMailHandler(app *app.App) *mailHandler {
	return &mailHandler{
		mailservice: mailServ.NewMailServiceFromEnv(),
	}
}

func (h *mailHandler) SendMail(ctx context.Context, req *mail.SendRequest) (*emptypb.Empty, error) {
	if err := h.mailservice.SendEmail(req.To, *req.Subject, *req.Body); err != nil {
		return nil, status.Error(codes.Internal, err.Error())
	}
	return &emptypb.Empty{}, nil
}

func (h *mailHandler) Setup(gRPCServer *grpc.Server) {
	mail.RegisterMailServiceServer(gRPCServer, h)
}
