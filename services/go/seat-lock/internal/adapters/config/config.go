package config

import (
	"context"
	"github.com/redis/go-redis/v9"
	"gitlab.com/drop-table-prod/backend/services/go/seat-lock/internal/adapters/logger"
	"os"
)

type Config struct {
	DB *redis.Client
}

func Configure() *Config {
	logger.New(
		true,
		"GMT+3",
	)
	logger.Log.Debugf("Debug mode: %t", true)

	logger.Log.Info("Connecting to redis..")

	conn := redis.NewClient(&redis.Options{
		Addr:     os.Getenv("REDIS_HOST"),
		Password: os.Getenv("REDIS_PASSWORD"),
		DB:       0,
	})

	pong, err := conn.Ping(context.Background()).Result()
	if err != nil {
		logger.Log.Fatal(err)
		os.Exit(1)
	}
	logger.Log.Debugf("Got Redis response: %s", pong)

	logger.Log.Debug("Connected to redis")

	logger.Log.Info("Configure success")

	return &Config{
		DB: conn,
	}
}
