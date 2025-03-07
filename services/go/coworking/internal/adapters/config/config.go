package config

import (
	"context"
	"fmt"
	"github.com/jackc/pgx/v5/pgxpool"
	"gitlab.com/drop-table-prod/backend/services/go/coworking/internal/adapters/logger"
	"gitlab.com/drop-table-prod/backend/services/go/coworking/internal/domain/utils/dotenv"
	"os"
)

type Config struct {
	DB *pgxpool.Pool
}

func Configure() *Config {
	logger.New(
		true,
		"GMT+3",
	)
	logger.Log.Debugf("Debug mode: %t", true)

	logger.Log.Info("Connecting to database...")

	url := fmt.Sprintf("postgres://%s:%s@%s:%s/%s?sslmode=disable",
		dotenv.GetEnv("DB_USER", "user"),
		dotenv.GetEnv("DB_PASSWORD", "password"),
		dotenv.GetEnv("DB_HOST", "localhost"),
		dotenv.GetEnv("DB_PORT", "5432"),
		dotenv.GetEnv("DB_DATABASE", "coworking"))

	logger.Log.Debugf("Connecting to %s", url)

	conn, err := pgxpool.New(context.Background(), url)
	if err != nil {
		logger.Log.Fatal(err)
		os.Exit(1)
	}

	logger.Log.Debug("Connected to database")

	logger.Log.Info("Configure success")

	return &Config{
		DB: conn,
	}
}
