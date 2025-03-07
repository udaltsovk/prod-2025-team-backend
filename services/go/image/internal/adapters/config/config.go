package config

import (
	"os"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/adapters/logger"
	"gitlab.com/drop-table-prod/backend/services/go/image/internal/domain/utils/dotenv"
)

type Config struct {
	Minio *minio.Client
}

func Configure() *Config {
	logger.New(
		true,
		"GMT+3",
	)
	logger.Log.Debugf("Debug mode: %t", true)

	logger.Log.Info("Initializing minio...")
	endpoint := dotenv.GetEnv("MINIO_HOST", "localhost")
	accessKey := dotenv.GetEnv("MINIO_LOGIN", "root")
	secretKey := dotenv.GetEnv("MINIO_PASSWORD", "beetroot")

	minioClient, err := minio.New(endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(accessKey, secretKey, ""),
		Secure: false,
	})

	if err != nil {
		logger.Log.Fatal(err)
		os.Exit(1)
	}

	logger.Log.Info("Minio initialized")

	logger.Log.Info("Configure success")

	return &Config{
		Minio: minioClient,
	}
}
