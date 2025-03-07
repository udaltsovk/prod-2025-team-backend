package app

import (
	"context"
	"fmt"
	"gitlab.com/drop-table-prod/backend/libs/go/compression"
	"net"

	"github.com/grpc-ecosystem/go-grpc-middleware/v2/interceptors/logging"
	"github.com/grpc-ecosystem/go-grpc-middleware/v2/interceptors/recovery"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/adapters/config"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/adapters/logger"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/utils/dotenv"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/encoding"
	"google.golang.org/grpc/status"
	"gorm.io/gorm"
)

// App содержит gRPC сервер и базу данных
type App struct {
	DB         *gorm.DB
	GRPCServer *grpc.Server
}

// InterceptorLogger — адаптация zap-логгера для gRPC
func InterceptorLogger(l *zap.SugaredLogger) logging.Logger {
	return logging.LoggerFunc(func(ctx context.Context, lvl logging.Level, msg string, fields ...any) {
		interceptorLogger := l.WithOptions(zap.AddCallerSkip(1)).With(fields...)
		switch lvl {
		case logging.LevelDebug:
			interceptorLogger.Debug(msg)
		case logging.LevelInfo:
			interceptorLogger.Info(msg)
		case logging.LevelWarn:
			interceptorLogger.Warn(msg)
		case logging.LevelError:
			interceptorLogger.Error(msg)
		default:
			panic(fmt.Sprintf("unknown level %v", lvl))
		}
	})
}

// New создаёт новый gRPC сервер
func New(config *config.Config) *App {
	encoding.RegisterCompressor(compression.ZstdCompressor{})

	loggingOpts := []logging.Option{
		logging.WithLogOnEvents(
			logging.PayloadReceived, logging.PayloadSent,
		),
	}

	recoveryOpts := []recovery.Option{
		recovery.WithRecoveryHandler(func(p interface{}) (err error) {
			logger.Log.Errorf("Recovered from panic: %v", p)
			return status.Errorf(codes.Internal, "internal error")
		}),
	}

	GRPCServer := grpc.NewServer(
		grpc.ChainUnaryInterceptor(
			recovery.UnaryServerInterceptor(recoveryOpts...),
			logging.UnaryServerInterceptor(InterceptorLogger(logger.Log.SugaredLogger), loggingOpts...),
		),
	)

	return &App{
		GRPCServer: GRPCServer,
		DB:         config.Database,
	}
}

// Start запускает сервер
func (a *App) Start() {
	logger.Log.Info("Starting gRPC server...")

	listener, err := net.Listen("tcp", fmt.Sprintf(":%s", dotenv.GetEnv("PORT", "50054")))

	if err != nil {
		logger.Log.Fatalf("failed to listen: %v", err)
	} else {
		logger.Log.Info("Listening on port " + dotenv.GetEnv("PORT", "50054"))
	}

	if errGRPCStart := a.GRPCServer.Serve(listener); errGRPCStart != nil {
		logger.Log.Fatalf("failed to serve: %v", errGRPCStart)
	}
}

// Stop останавливает сервер
func (a *App) Stop() {
	logger.Log.Info("Shutting down gRPC server...")
	a.GRPCServer.GracefulStop()
}
