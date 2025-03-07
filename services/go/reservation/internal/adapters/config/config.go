package config

import (
	"fmt"
	"log"
	"os"
	"time"

	"github.com/spf13/viper"
	postgresRepo "gitlab.com/drop-table-prod/backend/services/go/reservation/internal/adapters/database/postgres"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/adapters/logger"
	"gitlab.com/drop-table-prod/backend/services/go/reservation/internal/domain/utils/dotenv"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	gormLogger "gorm.io/gorm/logger"
)

type Config struct {
	Database *gorm.DB
}

func Configure() *Config {
	logger.New(true, "GMT+3")
	logger.Log.Debugf("Debug mode: %t", viper.GetBool("settings.debug"))

	newLogger := gormLogger.New(
		log.New(os.Stdout, "\r\n", log.LstdFlags),
		gormLogger.Config{
			SlowThreshold: time.Second,
			LogLevel:      gormLogger.Info,
			Colorful:      true,
		},
	)

	// Подключение к PostgreSQL без указания БД (по умолчанию "postgres")
	dsnBase := fmt.Sprintf("host=%s user=%s password=%s dbname=postgres port=%s sslmode=disable",
		dotenv.GetEnv("POSTGRES_HOST", "localhost"),
		dotenv.GetEnv("POSTGRES_USERNAME", "postgres"),
		dotenv.GetEnv("POSTGRES_PASSWORD", "postgres"),
		dotenv.GetEnv("POSTGRES_PORT", "5432"),
	)

	baseDB, err := gorm.Open(postgres.Open(dsnBase), &gorm.Config{Logger: newLogger})
	if err != nil {
		logger.Log.Panicf("Failed to connect to postgres: %v", err)
	}
	sqlDB, _ := baseDB.DB()

	// Проверяем, существует ли база данных "reservation", если нет - создаём
	var exists bool
	checkQuery := "SELECT EXISTS (SELECT 1 FROM pg_database WHERE datname = 'reservation');"
	err = baseDB.Raw(checkQuery).Scan(&exists).Error
	if err != nil {
		logger.Log.Panicf("Failed to check database existence: %v", err)
	}

	if !exists {
		logger.Log.Info("Database 'reservation' does not exist, creating...")
		if err := baseDB.Exec("CREATE DATABASE reservation").Error; err != nil {
			logger.Log.Panicf("Failed to create database: %v", err)
		}
		logger.Log.Info("Database 'reservation' created successfully.")
	}

	sqlDB.Close() // Закрываем соединение перед переподключением

	// Переподключаемся уже к созданной БД "reservation"
	dsnReservation := fmt.Sprintf("host=%s user=%s password=%s dbname=reservation port=%s sslmode=disable",
		dotenv.GetEnv("POSTGRES_HOST", "localhost"),
		dotenv.GetEnv("POSTGRES_USERNAME", "postgres"),
		dotenv.GetEnv("POSTGRES_PASSWORD", "postgres"),
		dotenv.GetEnv("POSTGRES_PORT", "5432"),
	)

	database, errConnect := gorm.Open(postgres.Open(dsnReservation), &gorm.Config{Logger: newLogger})
	if errConnect != nil {
		logger.Log.Panicf("Failed to connect to database 'reservation': %v", errConnect)
	} else {
		logger.Log.Info("Connected to database 'reservation'")
	}

	logger.Log.Info("Running migrations...")
	if errMigrate := database.AutoMigrate(postgresRepo.Migrations...); errMigrate != nil {
		logger.Log.Panicf("Failed to run migrations: %v", errMigrate)
	}

	sqlDB, _ = database.DB()
	sqlDB.SetMaxIdleConns(0)

	return &Config{Database: database}
}
