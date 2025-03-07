package app

import (
	"context"
	"fmt"
	"github.com/grpc-ecosystem/go-grpc-middleware/v2/interceptors/logging"
	"github.com/grpc-ecosystem/go-grpc-middleware/v2/interceptors/recovery"
	"github.com/redis/go-redis/v9"
	"gitlab.com/drop-table-prod/backend/libs/go/compression"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/adapters/config"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/adapters/logger"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/domain/utils/dotenv"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/encoding"
	"google.golang.org/grpc/status"
	"net"
)

// App is a struct that contains the fiber app, database connection, listen port, validator, logging boolean etc.
type App struct {
	GRPCServer *grpc.Server
	DB         *redis.Client
}

// InterceptorLogger is a func to convert zap logger to grpc logger
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

// New is a function that creates a new app struct
func New(config *config.Config) *App {
	encoding.RegisterCompressor(compression.ZstdCompressor{})

	loggingOpts := []logging.Option{
		logging.WithLogOnEvents(
			logging.PayloadReceived, logging.PayloadSent,
		),
	}

	recoveryOpts := []recovery.Option{
		recovery.WithRecoveryHandler(func(p interface{}) (err error) {
			// Логируем информацию о панике с уровнем Error
			logger.Log.Errorf("Recovered from panic: %v", p)

			// Можете либо честно вернуть клиенту содержимое паники
			// Либо ответить - "internal error", если не хотим делиться внутренностями
			return status.Errorf(codes.Internal, "internal error")
		}),
	}

	GRPCServer := grpc.NewServer(grpc.ChainUnaryInterceptor(
		recovery.UnaryServerInterceptor(recoveryOpts...),
		logging.UnaryServerInterceptor(InterceptorLogger(logger.Log.SugaredLogger), loggingOpts...),
	))
	return &App{
		GRPCServer: GRPCServer,
		DB:         config.DB,
	}
}

func (a *App) Start() {
	logger.Log.Info("Starting gRPC server...")
	port := dotenv.GetEnv("SERVER_PORT", "50055")
	listener, err := net.Listen("tcp", fmt.Sprintf(":%s", port))

	if err != nil {
		logger.Log.Fatalf("failed to listen: %v", err)
	} else {
		logger.Log.Info("Listening on port " + port)
	}

	if errGRPCStart := a.GRPCServer.Serve(listener); errGRPCStart != nil {
		logger.Log.Fatalf("failed to serve: %v", errGRPCStart)
	}
}

func (a *App) Stop() {
	logger.Log.Info("Shutting down gRPC server...")
	a.GRPCServer.GracefulStop()
}
